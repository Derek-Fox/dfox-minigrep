# (derek fox's) minigrep

`minigrep` is a simple command-line tool written in Rust for searching text within files, similar to the classic `grep` utility. This version expands on the implementation showcased [here](https://doc.rust-lang.org/book/ch12-00-an-io-project.html), in the 'Rust Book'. It supports colored output, line numbers, case-insensitive search, counting matches, quiet-mode, and can recurse through directory trees.

## Features

- Search for a query string in a file
- Optional colored highlighting of matches
- Optional display of line numbers
- Case-insensitive search option
- Quiet mode to suppress output
- Option to output the count of matches

## Usage

```sh
minigrep [OPTIONS] <query> <path>
```

- `<query>`: The string to search for (required)
- `<path>`: The file or directory to search in

## Options

- `--no-color`
  Disable colored output.

- `--no-lines`
  Disable line numbers in output.

- `--quiet`, `-q`
  Suppress all output.

- `--case-insensitive`, `-i`
  Perform a case-insensitive search.

- `--count`, `-c`
  Out the number of matches found.

## Examples

Search for "foo" in file.txt with colored output and line numbers:

```sh
minigrep foo file.txt
```

Recursively search for all matches in a directory:

```sh
minigrep foo dir/
```

Building
To build the project, run:

```sh
cargo build --release
```