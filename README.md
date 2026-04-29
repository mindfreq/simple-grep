`A simple grep-like tool written in Rust.`

## Installation

```bash
git clone https://github.com/mindfreq/simple-grep.git
cd simple-grep
cargo build --release
```

## Usage

```bash
# Basic search
s-grep <query> <file>

# Case-insensitive
s-grep -i <query> <file>
```

### Examples

```bash
s-grep "hello" src/main.rs
s-grep -i "WHO" poem.txt
```
