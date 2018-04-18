#![feature(test)]

extern crate test;

extern crate elasticlunr;

use test::Bencher;
use elasticlunr::Index;

// Results:
// std::collections::HashMap = 609,503 ns/iter (+/- 17,306)
// fnv::FnvHashMap = 612,780 ns/iter (+/- 26,132)

#[bench]
fn bench_add_doc(b: &mut Bencher) {
    let mut index = Index::new(&["title", "body"]);
    let mut i = 0;
    b.iter(|| {
        i += 1;
        index.add_doc(
            &i.to_string(),
            &[
                "This Week in Rust 206",
                "Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n
                Hello and welcome to another issue of This Week in Rust!\n"
            ],
        );
    });
}
