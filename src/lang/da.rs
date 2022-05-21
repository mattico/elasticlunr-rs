use super::{
    common::{RustStemmer, StopWordFilter, Trimmer},
    Language,
};
use crate::pipeline::Pipeline;
use rust_stemmers::Algorithm;

const WORDS: &'static [&'static str] = &[
    "", "ad", "af", "alle", "alt", "anden", "at", "blev", "blive", "bliver", "da", "de", "dem",
    "den", "denne", "der", "deres", "det", "dette", "dig", "din", "disse", "dog", "du", "efter",
    "eller", "en", "end", "er", "et", "for", "fra", "ham", "han", "hans", "har", "havde", "have",
    "hende", "hendes", "her", "hos", "hun", "hvad", "hvis", "hvor", "i", "ikke", "ind", "jeg",
    "jer", "jo", "kunne", "man", "mange", "med", "meget", "men", "mig", "min", "mine", "mit",
    "mod", "ned", "noget", "nogle", "nu", "når", "og", "også", "om", "op", "os", "over", "på",
    "selv", "sig", "sin", "sine", "sit", "skal", "skulle", "som", "sådan", "thi", "til", "ud",
    "under", "var", "vi", "vil", "ville", "vor", "være", "været",
];

const TRIM: &'static str =
    "A-Za-z\\xAA\\xBA\\xC0-\\xD6\\xD8-\\xF6\\xF8-\\u02B8\\u02E0-\\u02E4\\u1D00-\\u1D25\
    \\u1D2C-\\u1D5C\\u1D62-\\u1D65\\u1D6B-\\u1D77\\u1D79-\\u1DBE\\u1E00-\\u1EFF\\u2071\\u207F\
    \\u2090-\\u209C\\u212A\\u212B\\u2132\\u214E\\u2160-\\u2188\\u2C60-\\u2C7F\\uA722-\\uA787\
    \\uA78B-\\uA7AD\\uA7B0-\\uA7B7\\uA7F7-\\uA7FF\\uAB30-\\uAB5A\\uAB5C-\\uAB64\\uFB00-\\uFB06\
    \\uFF21-\\uFF3A\\uFF41-\\uFF5A";

pub struct Danish {
    stop_words: StopWordFilter,
    stemmer: RustStemmer,
    trimmer: Trimmer,
}

impl Danish {
    pub fn new() -> Self {
        let stop_words = StopWordFilter::new("stopWordFilter-da", WORDS);
        let stemmer = RustStemmer::new("stemmer-da", Algorithm::Danish);
        let trimmer = Trimmer::new("trimmer-da", TRIM);
        Self {
            stop_words,
            stemmer,
            trimmer,
        }
    }
}

impl Language for Danish {
    fn name(&self) -> String {
        "Danish".into()
    }
    fn code(&self) -> String {
        "da".into()
    }

    fn tokenize(&mut self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn pipeline(&mut self) -> Pipeline {
        Pipeline {
            queue: vec![self.trimmer, self.stop_words, self.stemmer],
        }
    }
}
