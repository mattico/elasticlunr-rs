#![cfg(feature = "bench")]
#![feature(test)]

extern crate test;
use test::Bencher;

extern crate elasticlunr;
use elasticlunr::Index;

// # Results
// HashMap: 3,612,098 ns/iter (+/- 303,386)

#[bench]
fn bench_holistic_en(b: &mut Bencher) {
    let text = include_str!("../tests/data/en.in.txt");
    let sections: Vec<_> = text.split("\n\n").collect();

    b.iter(|| {
        let mut index = Index::new(&["section"]);
        for (i, section) in sections.iter().enumerate() {
            index.add_doc(&format!("section_{}", i), &[section]);
        }
        test::black_box(index.to_json());
    });
}
