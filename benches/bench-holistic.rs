#![cfg(feature = "bench")]
#![feature(test)]

extern crate test;
use test::Bencher;

extern crate elasticlunr;
use elasticlunr::Index;

// # Results
// BTreeMap<String, IndexItem>: 3,165,389 ns/iter (+/- 420,869)
// BTreeMap<char, IndexItem>:   2,920,902 ns/iter (+/- 118,729)

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
