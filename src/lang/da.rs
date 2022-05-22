use super::{
    common::{RustStemmer, StopWordFilter, Trimmer},
    Language,
};
use crate::pipeline::Pipeline;
use rust_stemmers::Algorithm;

pub struct Danish {}

impl Danish {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for Danish {
    fn name(&self) -> String {
        "Danish".into()
    }
    fn code(&self) -> String {
        "da".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(Trimmer::new("trimmer-da", r"\p{Latin}")),
                Box::new(StopWordFilter::new("stopWordFilter-da", STOP_WORDS)),
                Box::new(RustStemmer::new("stemmer-da", Algorithm::Danish)),
            ],
        }
    }
}

const STOP_WORDS: &'static [&'static str] = &[
    "", "ad", "af", "alle", "alt", "anden", "at", "blev", "blive", "bliver", "da", "de", "dem",
    "den", "denne", "der", "deres", "det", "dette", "dig", "din", "disse", "dog", "du", "efter",
    "eller", "en", "end", "er", "et", "for", "fra", "ham", "han", "hans", "har", "havde", "have",
    "hende", "hendes", "her", "hos", "hun", "hvad", "hvis", "hvor", "i", "ikke", "ind", "jeg",
    "jer", "jo", "kunne", "man", "mange", "med", "meget", "men", "mig", "min", "mine", "mit",
    "mod", "ned", "noget", "nogle", "nu", "når", "og", "også", "om", "op", "os", "over", "på",
    "selv", "sig", "sin", "sine", "sit", "skal", "skulle", "som", "sådan", "thi", "til", "ud",
    "under", "var", "vi", "vil", "ville", "vor", "være", "været",
];
