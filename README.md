# Presentation Graphic Stream
Parsing Presentation Graphic Stream (BluRay Subtitle Format - SUP files)

[![Build Status][actions-badge]][actions-url]

[actions-badge]: https://github.com/mbolaric/pgs/actions/workflows/rust.yml/badge.svg?branch=master
[actions-url]: https://github.com/mbolaric/pgs/actions/workflows/rust.yml?query=branch%3Amaster

# Usage
```rust
use pgs_parse::PgsParser;

let parser = PgsParser::parse("subtitle.sup");
match parser {
    Ok(parser) => {
        let ds = parser.get_display_sets();
        // ...
    },
    Err(err) => {
        // ...
    }
}
```