// Input text is excerpted from public domain books on gutenberg.org or wikisource.org

extern crate elasticlunr;
extern crate strum;

use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Read, Write};

use elasticlunr::*;
use elasticlunr::pipeline::tokenize;
use strum::IntoEnumIterator;

fn get_lang_code(lang: Language) -> &'static str {
    match lang {
        #[cfg(feature = "du")] Language::Dutch => "du",
        _ => lang.to_code(),
    }
}

#[allow(dead_code)]
fn write_output(lang: Language) {
    let code = get_lang_code(lang);
    let base = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data");

    let input = base.join(&format!("{}.in.txt", code));
    let mut input_str = String::new();
    File::open(&input)
        .unwrap()
        .read_to_string(&mut input_str)
        .unwrap();

    let output = base.join(&format!("{}.out.rs.txt", code));
    let mut output = File::create(&output).unwrap();

    let pipeline = lang.make_pipeline();
    let tokens = pipeline.run(tokenize(&input_str));

    for tok in tokens {
        writeln!(&mut output, "{}", tok).unwrap();
    }
}

fn compare_to_fixture(lang: Language) {
    let code = get_lang_code(lang);
    let base = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data");

    let input = base.join(&format!("{}.in.txt", code));
    let mut input_str = String::new();
    File::open(&input)
        .unwrap()
        .read_to_string(&mut input_str)
        .unwrap();

    let output = base.join(&format!("{}.out.txt", code));
    let mut output = BufReader::new(File::open(&output).unwrap()).lines();

    let pipeline = lang.make_pipeline();
    let tokens = pipeline.run(tokenize(&input_str));

    for tok in tokens {
        assert_eq!(
            tok,
            output.next().unwrap().unwrap(),
            "Comparing pipeline tokens to fixture for {:?}",
            lang
        );
    }
}

#[test]
fn test_languages() {
    for lang in Language::iter() {
        //write_output(lang);
        compare_to_fixture(lang);
    }
}
