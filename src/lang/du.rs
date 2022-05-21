use super::{
    common::{RustStemmer, StopWordFilter, Trimmer},
    Language,
};
use crate::pipeline::Pipeline;
use rust_stemmers::{Algorithm, Stemmer};

const WORDS: &'static [&'static str] = &[
    "", "aan", "al", "alles", "als", "altijd", "andere", "ben", "bij", "daar", "dan", "dat", "de",
    "der", "deze", "die", "dit", "doch", "doen", "door", "dus", "een", "eens", "en", "er", "ge",
    "geen", "geweest", "haar", "had", "heb", "hebben", "heeft", "hem", "het", "hier", "hij", "hoe",
    "hun", "iemand", "iets", "ik", "in", "is", "ja", "je", "kan", "kon", "kunnen", "maar", "me",
    "meer", "men", "met", "mij", "mijn", "moet", "na", "naar", "niet", "niets", "nog", "nu", "of",
    "om", "omdat", "onder", "ons", "ook", "op", "over", "reeds", "te", "tegen", "toch", "toen",
    "tot", "u", "uit", "uw", "van", "veel", "voor", "want", "waren", "was", "wat", "werd", "wezen",
    "wie", "wil", "worden", "wordt", "zal", "ze", "zelf", "zich", "zij", "zijn", "zo", "zonder",
    "zou",
];

const TRIM: &'static str =
    "A-Za-z\\xAA\\xBA\\xC0-\\xD6\\xD8-\\xF6\\xF8-\\u02B8\\u02E0-\\u02E4\\u1D00-\\u1D25\
    \\u1D2C-\\u1D5C\\u1D62-\\u1D65\\u1D6B-\\u1D77\\u1D79-\\u1DBE\\u1E00-\\u1EFF\\u2071\\u207F\
    \\u2090-\\u209C\\u212A\\u212B\\u2132\\u214E\\u2160-\\u2188\\u2C60-\\u2C7F\\uA722-\\uA787\
    \\uA78B-\\uA7AD\\uA7B0-\\uA7B7\\uA7F7-\\uA7FF\\uAB30-\\uAB5A\\uAB5C-\\uAB64\\uFB00-\\uFB06\
    \\uFF21-\\uFF3A\\uFF41-\\uFF5A";

pub struct Dutch {
    trimmer: Trimmer,
    stop_words: StopWordFilter,
    stemmer: Stemmer,
}

impl Dutch {
    pub fn new() -> Self {
        let trimmer = Trimmer::new("trimmer-du", TRIM);
        let stop_words = StopWordFilter::new("stopWordFilter-du", WORDS);
        let stemmer = RustStemmer::new("stemmer-du", Algorithm::Dutch);
        Self {
            trimmer,
            stop_words,
            stemmer,
        }
    }
}

impl Language for Dutch {
    const NAME: &'static str = "Dutch";
    const CODE: &'static str = "du";

    fn tokenize(&mut self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn pipeline(&mut self) -> Pipeline {
        Pipeline {
            queue: vec![self.trimmer, self.stop_words, self.stemmer],
        }
    }
}
