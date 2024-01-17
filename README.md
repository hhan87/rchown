## r(ust)chown

This code is written by GPT-4.

Just for my own requirement to handle many files on NAS.

This program changes file owner recusively based on DFS algorithm.

And Next texts is created by GPT-4 and edited by me for README.md.

# R(ust)Chown

## Overview
`rchown` is a Rust-based command-line tool for changing file ownership recursively on NAS systems, using a DFS algorithm.

## Prerequisites
- Rust environment ([Installation guide](https://www.rust-lang.org/tools/install))
- NAS or file system access

## Installation
```bash
git clone https://github.com/hhan87/rchown.git
cd rchown
cargo build --release
```

## Usage
```bash
./target/release/rchown <user> <group> <target_directory>
```
- `<user>`: New Owner UserName
- `<group>`: New Owner GroupName
- `<target_directory>`: Directory for ownership change.


## License
[MIT License](LICENSE)
