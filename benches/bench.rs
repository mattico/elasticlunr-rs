use criterion::{black_box, criterion_group, criterion_main, Criterion};
use elasticlunr::{lang, pipeline::tokenize, Index};

fn bench_main(c: &mut Criterion) {
    // BTreeMap<String, IndexItem>: 3,165,389 ns/iter (+/- 420,869)
    // BTreeMap<char, IndexItem>:   2,920,902 ns/iter (+/- 118,729)
    c.bench_function("create_index", |b| {
        let text = include_str!("../tests/data/en.in.txt");
        let sections: Vec<_> = text.split("\n\n").collect();
        b.iter(|| {
            let mut index = Index::new(&["section"]);
            for (i, section) in sections.iter().enumerate() {
                index.add_doc(&format!("section_{}", i), &[section]);
            }
            black_box(index.to_json());
        })
    });

    // PHF:           2,507,473 ns/iter (+/- 197,005)
    // Linear search: 2,481,133 ns/iter (+/- 216,998)
    c.bench_function("stemmer", |b| {
        let text = include_str!("../tests/data/en.in.txt");
        let tokens = tokenize(text);
        b.iter(|| {
            let tokens = tokens.clone();
            for token in tokens {
                black_box(lang::en::stemmer(token));
            }
        });
    });

    // HashSet:  175,669 ns/iter (+/- 15,652)
    // BTreeSet: 210,169 ns/iter (+/- 29,430)
    // PHF:      159,961 ns/iter (+/- 16,492)
    #[cfg(feature = "fr")]
    c.bench_function("stop_word_fr", |b| {
        let text = include_str!("../tests/data/fr.in.txt");
        let tokens = tokenize(text);
        let tokens: Vec<_> = tokens
            .into_iter()
            .filter_map(|t| lang::fr::trimmer(t))
            .collect();

        b.iter(|| {
            let tokens = tokens.clone();
            for token in tokens {
                black_box(lang::fr::stop_word_filter(token));
            }
        });
    });
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
