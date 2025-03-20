#![no_main]

use elasticlunr::lang::{English, Language};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // libfuzz was finding a ton of UTF-8 inputs which caused panics. That's worth addressing, but it needs to be
        // done with an eye towards compatilibity with elasticlunr.js. Punt for now.
        if !s.is_ascii() {
            return;
        }
        let en = English::new();
        let pipeline = en.make_pipeline();
        let tokens = en.tokenize(s);
        let filtered = pipeline.run(tokens);
        std::hint::black_box(filtered);
    }
});
