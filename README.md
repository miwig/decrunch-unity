# Decoder for crunch-compressed texture data
[![Crates.io](https://img.shields.io/crates/v/decrunch.svg)](https://crates.io/crates/decrunch-unity)
[![Documentation](https://docs.rs/decrunch-unity/badge.svg)](https://docs.rs/decrunch-unity)

This crate provides a Rust wrapper around the [Unity fork](https://github.com/Unity-Technologies/crunch) of the crunch decompressor.

# Example

```rust
use decrunch::*;
use std::fs::File;
use std::io::Read;

let mut compressed_file = File::open("testdata/copyright_2048_compressed.dat")?;
let mut compressed_data = Vec::new();

compressed_file.read_to_end(&mut compressed_data)?;

let c_data = CrunchedData::new(&compressed_data);
let decompressed_data = match c_data.decode_level(0) {
    None => {
        panic!("Failed to decompress texture data");
    }
    Some(res) => res,
};

assert!(decompressed_data.len() > 0);
```
