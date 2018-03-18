//! # Language Support
//!
//! Ported from <https://github.com/weixsong/lunr-languages>. Each supported language has a trimmer,
//! a stop word filter, and a stemmer. Use [`Pipeline::for_language`](../pipeline/struct.Pipeline.html#for_language)
//! to create a corresponding pipeline.

macro_rules! make_trimmer {
    ($reg:expr) => {
        pub fn trimmer(token: String) -> Option<String> {
            use ::regex::Regex;
            lazy_static! {
                static ref START: Regex = Regex::new(concat!("^[^", $reg, "]+")).unwrap();
                static ref END: Regex = Regex::new(concat!("[^", $reg, "]+$")).unwrap();
            }
            let token = START.replace(&token, "");
            Some(END.replace(&token, "").into())
        }
    };
}

macro_rules! make_stop_word_filter {
    ($words:expr) => {
        pub fn stop_word_filter(token: String) -> Option<String> {
            use ::std::collections::HashSet;
            lazy_static! {
                static ref WORDS: HashSet<&'static str> = {
                    let words = $words;
                    let mut set = HashSet::with_capacity(words.len());
                    for word in words.iter() {
                        set.insert(*word);
                    }
                    set
                };
            }
            if WORDS.contains(token.as_str()) {
                None
            } else {
                Some(token)
            }
        }
    };
}

macro_rules! make_stemmer {
    ($lang:expr) => {
        pub fn stemmer(token: String) -> Option<String> {
            use rust_stemmers::{Algorithm, Stemmer};
            lazy_static! {
                static ref STEMMER: Stemmer = Stemmer::create($lang);
            }
            Some(STEMMER.stem(&token).into())
        }
    };
}

/// A list of the currently supported languages by their [ISO 639-1][iso] code.
/// [iso]: https://en.wikipedia.org/wiki/ISO_639-1
pub static SUPPORTED_LANGUAGES: &'static [&'static str] = &[
    "da", "de", "du", "en", "es", "fi", "fr", "hu", "it", "pt", "ro", "ru", "sv", "tr"
];

pub mod da;
pub mod de;
pub mod du;
pub mod en;
pub mod es;
pub mod fi;
pub mod fr;
pub mod hu;
pub mod it;
pub mod pt;
pub mod ro;
pub mod ru;
pub mod sv;
pub mod tr;
