# cattaca

`cattaca` reads from a specified file or STDIN and outputs a specified number of random lines from the input. `cattaca` will always output the requested number of lines.

## Arguments

FILE (optional): the file to use as a source, if not given then `cattaca` will read from STDIN

`-n`: the number of lines to output (defaults to 10)

## Usage Examples

```
# output seven random letters from "c" "a" "t"
$ echo "c\na\nt" | cattaca -n 7
c
a
t
t
a
c
a

# output 1000 random lines pulled from datafile.txt
$ cattaca datafile.txt -n 1000

# output 10 random lines from STDIN
$ rg checksum Cargo.lock | cattaca -n 10
checksum = "34af8d1a0e25924bc5b7c43c079c942339d8f0a8b57c39049bef581b46327404"
checksum = "2304e00983f87ffb38b55b444b5e3b60a884b5d30c0fca7d82fe33449bbe55ea"
checksum = "97b3888a4aecf77e811145cadf6eef5901f4782c53886191b2f693f24761847c"
checksum = "9c8d87e72b64a3b4db28d11ce29237c246188f4f51057d65a7eab63b7987e423"
checksum = "0b6a852b24ab71dffc585bcb46eaf7959d175cb865a7152e35b348d1b2960422"
checksum = "34af8d1a0e25924bc5b7c43c079c942339d8f0a8b57c39049bef581b46327404"
checksum = "528131438037fd55894f62d6e9f068b8f45ac57ffa77517819645d10aed04f64"
checksum = "cf4b9d6a944f767f8e5e0db018570623c85f3d925ac718db4e06d0187adb21c1"
checksum = "bec47e5bfd1bff0eeaf6d8b485cc1074891a197ab4225d504cb7a1ab88b02bf0"
checksum = "ec0be4795e2f6a28069bec0b5ff3e2ac9bafc99e6a9a7dc3547996c5c816922c"
```

## Installation (macOS)

### Straight from the tap

```
brew install sdball/tap/cattaca
```

### Setup tap for future use

First setup my homebrew tap

```
brew tap sdball/tap
```

Then install `cattaca`

```
brew install cattaca
```
