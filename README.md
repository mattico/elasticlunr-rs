# elasticlunr-rs 

[![Build Status](https://travis-ci.org/mattico/elasticlunr-rs.svg?branch=master)](https://travis-ci.org/mattico/elasticlunr-rs) [![Documentation](https://docs.rs/elasticlunr-rs/badge.svg)](https://docs.rs/elasticlunr-rs) [![Crates.io](https://img.shields.io/crates/v/elasticlunr-rs.svg)](https://crates.io/crates/elasticlunr-rs)

A partial port of [elasticlunr](https://github.com/weixsong/elasticlunr.js) to Rust. Intended to be used for generating compatible search indices.

## Usage

```Rust
let mut index_builder = IndexBuilder::new();
index_builder.add_document("This Week in Rust 207",
    "Hello and welcome to another issue of This Week in Rust!");
index_builder.add_document("This Week in Rust 206",
    "Hello and welcome to another issue of This Week in Rust!");
let mut file = fs::File::create("out.json").unwrap();
file.write_all(index_builder.to_json_pretty().as_bytes());
```

## License

This repository is offered under the terms of the

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
