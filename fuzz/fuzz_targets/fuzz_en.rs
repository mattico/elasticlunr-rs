#![no_main]

use elasticlunr::lang::{English, Language};
use libfuzzer_sys::fuzz_target;

// Non-ASCII input matters here: the stemmer indexes its buffer by byte, so no
// rule may split a multi-byte character.
//
// Taking `&str` rather than `&[u8]` lets `arbitrary` hand the target valid UTF-8
// directly. That runs ~35% fewer executions per second than decoding a byte
// slice and discarding what fails, but spends all of them inside the pipeline
// rather than on the rejection path, and covers measurably more edges.
fuzz_target!(|s: &str| {
    let en = English::new();
    let pipeline = en.make_pipeline();
    let tokens = en.tokenize(s);
    let filtered = pipeline.run(tokens);
    std::hint::black_box(filtered);
});
