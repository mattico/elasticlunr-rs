use std::ascii::AsciiExt;
use rust_stemmers;

pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|s| String::from(s.to_ascii_lowercase()))
        .collect()
}

pub type PipelineFn = fn(String) -> Option<String>;

#[derive(Serialize)]
pub struct Pipeline {
    #[serde(skip)]
    queue_fns: Vec<PipelineFn>,
    #[serde(rename = "_queue")]
    queue_names: Vec<String>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Pipeline {
            queue_fns: vec![trimmer, stop_word_filter, stemmer],
            queue_names: vec!["trimmer".into(), "stopWordFilter".into(), "stemmer".into()],
        }
    }
}

impl Pipeline {
    pub fn empty() -> Self {
        Pipeline {
            queue_fns: vec![],
            queue_names: vec![],
        }
    }

    // TODO: before() after(), etc.

    pub fn register_function(&mut self, name: String, func: PipelineFn) {
        self.queue_fns.push(func);
        self.queue_names.push(name);
    }

    // Could return impl Iterator<Item=String>
    pub fn run(&self, tokens: Vec<String>) -> Vec<String> {
        let mut ret = vec![];
        for token in tokens {
            let mut token = Some(token);
            for &func in &self.queue_fns {
                if let Some(t) = token {
                    token = func(t);
                } else {
                    break;
                }
            }
            if let Some(t) = token {
                ret.push(t);
            }
        }
        ret
    }
}

fn trimmer(token: String) -> Option<String> {
    Some(token.trim().into())
}

// TODO: languages
fn stemmer(token: String) -> Option<String> {
    lazy_static! {
        static ref STEMMER: rust_stemmers::Stemmer = 
            rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English);
    }

    Some(STEMMER.stem(&token).into())
}

// TODO: languages
fn stop_word_filter(token: String) -> Option<String> {
    match STOP_WORDS.contains(token.as_str()) {
        true => None,
        false => Some(token),
    }
}

static STOP_WORDS: ::phf::Set<&'static str> = phf_set! {
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
};
