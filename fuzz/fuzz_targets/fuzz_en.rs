#![no_main]

use elasticlunr::lang::{English, Language};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let en = English::new();
        let pipeline = en.make_pipeline();
        let tokens = en.tokenize(s);
        let filtered = pipeline.run(tokens);
        std::hint::black_box(filtered);
    }
});
