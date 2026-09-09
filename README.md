# Presentation Graphic Stream (pgs-parse)
A Rust library for parsing and decoding Blu-ray Presentation Graphic Stream (PGS/SUP) subtitle files.

[![Build Status][actions-badge]][actions-url]
[![Crates.io][crate-badge]][crate-url]

[actions-badge]: https://github.com/mbolaric/pgs/actions/workflows/rust.yml/badge.svg?branch=master
[actions-url]: https://github.com/mbolaric/pgs/actions/workflows/rust.yml?query=branch%3Amaster
[crate-badge]: https://img.shields.io/crates/v/pgs-parse.svg
[crate-url]: https://crates.io/crates/pgs-parse

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
pgs-parse = "0.1.2"
```

## Usage

```rust
use pgs_parse::{PgsParser, PgsDisplaySetState};

let parser = PgsParser::parse("subtitles.sup")?;

for ds in parser.get_display_sets() {
    if ds.state() == PgsDisplaySetState::Complete {
        // Decode RLE image: true for grayscale (ideal for OCR), false for 32-bit ARGB
        let pixels: Vec<Vec<u32>> = ds.get_decoded_image(true)?;
        let ods = ds.ods.as_ref().unwrap();
        println!("Subtitle frame: {}x{}", ods.width, ods.height);
    }
}
```

## Running Examples

Export a subtitle frame to a TIFF image:
```bash
cargo run --example parse_sup_file -- -p examples/data/BluRay.sup -t output.tiff -d 0
```

Read and inspect segments from a SUP file:
```bash
cargo run --example read_sup_file
```

## Testing
```bash
cargo test
```