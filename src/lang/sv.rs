use super::{
    common::{RustStemmer, StopWordFilter, Trimmer},
    Language,
};
use crate::pipeline::Pipeline;
use rust_stemmers::Algorithm;

pub struct Swedish {}

impl Swedish {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for Swedish {
    fn name(&self) -> String {
        "Swedish".into()
    }
    fn code(&self) -> String {
        "sv".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(Trimmer::new("trimmer-sv", WORD_CHARS)),
                Box::new(StopWordFilter::new("stopWordFilter-sv", STOP_WORDS)),
                Box::new(RustStemmer::new("stemmer-sv", Algorithm::Swedish)),
            ],
        }
    }
}
const WORD_CHARS: &'static str =
    "A-Za-z\\xAA\\xBA\\xC0-\\xD6\\xD8-\\xF6\\xF8-\\u02B8\\u02E0-\\u02E4\\u1D00-\\u1D25\
     \\u1D2C-\\u1D5C\\u1D62-\\u1D65\\u1D6B-\\u1D77\\u1D79-\\u1DBE\\u1E00-\\u1EFF\\u2071\\u207F\
     \\u2090-\\u209C\\u212A\\u212B\\u2132\\u214E\\u2160-\\u2188\\u2C60-\\u2C7F\\uA722-\\uA787\
     \\uA78B-\\uA7AD\\uA7B0-\\uA7B7\\uA7F7-\\uA7FF\\uAB30-\\uAB5A\\uAB5C-\\uAB64\\uFB00-\\uFB06\
     \\uFF21-\\uFF3A\\uFF41-\\uFF5A";

const STOP_WORDS: &'static [&'static str] = &[
    "", "alla", "allt", "att", "av", "blev", "bli", "blir", "blivit", "de", "dem", "den", "denna",
    "deras", "dess", "dessa", "det", "detta", "dig", "din", "dina", "ditt", "du", "där", "då",
    "efter", "ej", "eller", "en", "er", "era", "ert", "ett", "från", "för", "ha", "hade", "han",
    "hans", "har", "henne", "hennes", "hon", "honom", "hur", "här", "i", "icke", "ingen", "inom",
    "inte", "jag", "ju", "kan", "kunde", "man", "med", "mellan", "men", "mig", "min", "mina",
    "mitt", "mot", "mycket", "ni", "nu", "när", "någon", "något", "några", "och", "om", "oss",
    "på", "samma", "sedan", "sig", "sin", "sina", "sitta", "själv", "skulle", "som", "så", "sådan",
    "sådana", "sådant", "till", "under", "upp", "ut", "utan", "vad", "var", "vara", "varför",
    "varit", "varje", "vars", "vart", "vem", "vi", "vid", "vilka", "vilkas", "vilken", "vilket",
    "vår", "våra", "vårt", "än", "är", "åt", "över",
];
