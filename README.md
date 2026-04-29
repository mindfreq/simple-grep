# m-grep 🔍

A simple grep-like tool written in Rust.

## Installation

```bash
git https://codeberg.org/bux/m-grep.git
cd m-grep
cargo build --release
```

## Usage

```bash
# Basic search
m-grep <query> <file>

# Case-insensitive
m-grep -i <query> <file>
```

### Examples

```bash
m-grep "hello" src/main.rs
m-grep -i "WHO" poem.txt
```
