use clap::Parser;
use rand::Rng;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek, SeekFrom},
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

#[derive(Parser)]
#[command(version, about = "Output a given number of random lines from a file or STDIN", long_about = None)]
struct Cli {
    /// Optional filename
    filename: Option<PathBuf>,

    /// Number of lines to output
    #[arg(short, long, value_name = "NUMBER", default_value_t = 10)]
    number: usize,
}

fn main() {
    let cli = Cli::parse();
    // Declare a variable to hold the temporary file if created
    let temp_file_holder;

    let input_file_path = match cli.filename.as_deref() {
        Some(path) => path.to_path_buf(),
        None => {
            // Create a temporary file and keep it open
            let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            io::copy(&mut handle, &mut temp_file).expect("Failed to write STDIN to temp file");
            // Store the temp file to keep it in scope
            temp_file_holder = Some(temp_file);
            temp_file_holder.as_ref().unwrap().path().to_path_buf()
        }
    };

    print_random_lines(&input_file_path, cli.number);
}

fn print_random_lines(file: &Path, number: usize) {
    let f = File::open(file)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", file.display(), e));
    let mut f = BufReader::new(f);

    let file_len = f.get_ref().metadata().unwrap().len();
    let mut rng = rand::thread_rng();

    for _ in 0..number {
        // Seek to a random position in the file
        let random_pos = rng.gen_range(0..file_len);
        f.seek(SeekFrom::Start(random_pos))
            .expect("Couldn't seek to position");

        // Read forward to the next newline
        let mut buf = String::new();
        f.read_line(&mut buf).expect("Couldn't read line");

        // Read the line
        buf.clear();
        let res = f.read_line(&mut buf).expect("Couldn't read line");
        // Wrap around to the beginning of the file if we've hit the end
        if res == 0 {
            f.seek(SeekFrom::Start(0))
                .expect("Couldn't seek to start of file");
            f.read_line(&mut buf).expect("Couldn't read first line");
        }
        println!("{}", buf.trim_end());
    }
}
