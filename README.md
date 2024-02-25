# File Compression and Decompression in Rust

This project provides functionality to compress and decompress files using the Gzip format in Rust.

## Features

- Compress a file
- Decompress a file

## Usage

### Compress a File

To compress a file, pass the `compress` command with the source file and target file as arguments:

```rust
cargo run compress <source_file> <target_file>
```

This will compress the source file and save the compressed data to target file.

### Decompress a File

To decompress a file, call the `uncompress` command with the source file and target file as arguments:

```rust
cargo run uncompress <source_file> <target_file>
```

This will uncompress the source file and save the uncompressed data to target file.

### Dependencies
This project uses the flate2 crate for compression and decompression. Make sure to add it to your Cargo.toml file:

```
[dependencies]
flate2 = "1.0.24"
```