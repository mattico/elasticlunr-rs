# elasticlunr-rs 

[![Build Status](https://travis-ci.org/mattico/elasticlunr-rs.svg?branch=master)](https://travis-ci.org/mattico/elasticlunr-rs)
[![Documentation](https://docs.rs/elasticlunr-rs/badge.svg)](https://docs.rs/elasticlunr-rs)
[![Crates.io](https://img.shields.io/crates/v/elasticlunr-rs.svg)](https://crates.io/crates/elasticlunr-rs)

A partial port of [elasticlunr.js][eljs] to Rust. Intended to be used for 
generating compatible search indices.

## Example

```Rust
use std::fs::File;
use std::io::Write;
use elasticlunr::Index;

let mut index = Index::new(&["title", "body"]);
index.add_doc("1", &["This is a title", "This is body text!"]);
// Add more documents...
let mut file = File::create("out.json").unwrap();
file.write_all(index.to_json_pretty().as_bytes());
```

## License

This repository is offered under the terms of the

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted 
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Includes code ported from [elasticlunr.js][eljs] Copyright (C) 2017 by Wei Song, 
used under license. See LICENSE-JS for details.

Includes stop word lists ported from [stopwords-filter][swft] Copyright (C) 2012 
David J. Brenes, used under license. See LICENSE-WORDS for details.

[eljs]: https://github.com/weixsong/elasticlunr.js
[swft]: https://github.com/brenes/stopwords-filter
