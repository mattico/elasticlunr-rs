use crate::pipeline::PipelineFn;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashSet;

pub struct StopWordFilter {
    name: String,
    stop_words: HashSet<String>,
}

impl StopWordFilter {
    pub fn new(name: &str, stop_words: &[&str]) -> Self {
        Self {
            name: name.into(),
            stop_words: stop_words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl PipelineFn for StopWordFilter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn filter(&mut self, token: String) -> Option<String> {
        if self.stop_words.contains(&token) {
            None
        } else {
            Some(token)
        }
    }
}

pub struct Trimmer {
    name: String,
    trimmer: Regex,
}

impl Trimmer {
    pub fn new(name: &str, trim_chars: &str) -> Self {
        let name = name.into();
        let trimmer = Regex::new(&format!("^[^{0}]+|[^{0}]+$", trim_chars)).unwrap();
        Self { name, trimmer }
    }
}

impl PipelineFn for Trimmer {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn filter(&mut self, token: String) -> Option<String> {
        let result = self.trimmer.replace_all(&token, "");
        if result.is_empty() {
            None
        } else if result == token {
            Some(token)
        } else {
            Some(result.into())
        }
    }
}

pub struct RustStemmer {
    name: String,
    stemmer: Stemmer,
}

impl RustStemmer {
    pub fn new(name: &str, algo: Algorithm) -> Self {
        Self {
            name: name.into(),
            stemmer: Stemmer::create(algo),
        }
    }
}

impl PipelineFn for RustStemmer {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn filter(&mut self, token: String) -> Option<String> {
        let result = self.stemmer.stem(&token);
        if result.is_empty() {
            None
        } else if result == token {
            Some(token)
        } else {
            Some(result.into())
        }
    }
}
