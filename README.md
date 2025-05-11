# list-mess

`list-mess` is a small command-line utility to list the mess in a code project directory, written in
Rust. What are considered mess are typically files outside a Git repository, or Git repositories
that are not in a "clean" state.

## Requirements

To install `list-mess`, you simply need to have [Rust](https://www.rust-lang.org/) installed.

## Installation

To install `list-mess`, first use the following command to build the project binary:

```bash
cargo build --release
```

Then, you can copy the binary to a directory in your `PATH`, e.g.:

```bash
cp target/release/list-mess ~/.local/bin/
```

Optionally, create an alias in your shell configuration file:

```bash
alias lm='list-mess'
```
