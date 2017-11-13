extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    generate_stop_word_phf();
    generate_stemmer_phfs();
}

fn generate_stop_word_phf() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("stop_words.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(
        &mut file,
        "pub static STOP_WORDS: ::phf::Set<&'static str> = "
    ).unwrap();
    let mut set = phf_codegen::Set::new();

    for word in STOP_WORDS {
        set.entry(*word);
    }

    set.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}

fn generate_stemmer_phfs() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("stemmer_maps.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    {
        write!(
            &mut file,
            "pub static STEMMER_STEP_2: ::phf::Map<&'static str, &'static str> = "
        ).unwrap();
        let mut map = phf_codegen::Map::new();

        for &(w1, w2) in STEMMER_STEP_2 {
            map.entry(w1, w2);
        }

        map.build(&mut file).unwrap();
        write!(&mut file, ";\n").unwrap();
    }

    {
        write!(
            &mut file,
            "pub static STEMMER_STEP_3: ::phf::Map<&'static str, &'static str> = "
        ).unwrap();
        let mut map = phf_codegen::Map::new();

        for &(w1, w2) in STEMMER_STEP_3 {
            map.entry(w1, w2);
        }

        map.build(&mut file).unwrap();
        write!(&mut file, ";\n").unwrap();
    }
}

static STOP_WORDS: &[&str] = &[
    "",
    "a",
    "able",
    "about",
    "across",
    "after",
    "all",
    "almost",
    "also",
    "am",
    "among",
    "an",
    "and",
    "any",
    "are",
    "as",
    "at",
    "be",
    "because",
    "been",
    "but",
    "by",
    "can",
    "cannot",
    "could",
    "dear",
    "did",
    "do",
    "does",
    "either",
    "else",
    "ever",
    "every",
    "for",
    "from",
    "get",
    "got",
    "had",
    "has",
    "have",
    "he",
    "her",
    "hers",
    "him",
    "his",
    "how",
    "however",
    "i",
    "if",
    "in",
    "into",
    "is",
    "it",
    "its",
    "just",
    "least",
    "let",
    "like",
    "likely",
    "may",
    "me",
    "might",
    "most",
    "must",
    "my",
    "neither",
    "no",
    "nor",
    "not",
    "of",
    "off",
    "often",
    "on",
    "only",
    "or",
    "other",
    "our",
    "own",
    "rather",
    "said",
    "say",
    "says",
    "she",
    "should",
    "since",
    "so",
    "some",
    "than",
    "that",
    "the",
    "their",
    "them",
    "then",
    "there",
    "these",
    "they",
    "this",
    "tis",
    "to",
    "too",
    "twas",
    "us",
    "wants",
    "was",
    "we",
    "were",
    "what",
    "when",
    "where",
    "which",
    "while",
    "who",
    "whom",
    "why",
    "will",
    "with",
    "would",
    "yet",
    "you",
    "your",
];

// Need to surround values with \" because phf_codegen doesn't do it for us
// in the output file.

static STEMMER_STEP_2: &[(&str, &str)] = &[
    ("ational", "\"ate\""),
    ("tional", "\"tion\""),
    ("enci", "\"ence\""),
    ("anci", "\"ance\""),
    ("izer", "\"ize\""),
    ("bli", "\"ble\""),
    ("alli", "\"al\""),
    ("entli", "\"ent\""),
    ("eli", "\"e\""),
    ("ousli", "\"ous\""),
    ("ization", "\"ize\""),
    ("ation", "\"ate\""),
    ("ator", "\"ate\""),
    ("alism", "\"al\""),
    ("iveness", "\"ive\""),
    ("fulness", "\"ful\""),
    ("ousness", "\"ous\""),
    ("aliti", "\"al\""),
    ("iviti", "\"ive\""),
    ("biliti", "\"ble\""),
    ("logi", "\"log\""),
];

static STEMMER_STEP_3: &[(&str, &str)] = &[
    ("icate", "\"ic\""),
    ("ative", "\"\""),
    ("alize", "\"al\""),
    ("iciti", "\"ic\""),
    ("ical", "\"ic\""),
    ("ful", "\"\""),
    ("ness", "\"\""),
];
