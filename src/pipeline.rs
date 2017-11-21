// TODO: remove this when (https://github.com/rust-lang/rust/pull/44042) is stable
#[allow(unused_imports)]
use std::ascii::AsciiExt;
use serde::ser::{Serialize, Serializer, SerializeSeq};
pub use stemmer::stemmer;

pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| c.is_whitespace() || c == '-')
        .filter(|s| s.len() > 0)
        .map(|s| String::from(s.to_ascii_lowercase()))
        .collect()
}

pub type PipelineFn = fn(String) -> Option<String>;

#[derive(Debug)]
pub struct Pipeline {
    queue: Vec<(String, PipelineFn)>,
}

impl Serialize for Pipeline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.queue.len()))?;
        for &(ref name, _) in &self.queue {
            seq.serialize_element(&name)?;
        }
        seq.end()
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Pipeline {
            queue: vec![
                ("trimmer".into(), trimmer),
                ("stopWordFilter".into(), stop_word_filter),
                ("stemmer".into(), stemmer),
            ],
        }
    }
}

impl Pipeline {
    pub fn run(&self, tokens: Vec<String>) -> Vec<String> {
        let mut ret = vec![];
        for token in tokens {
            let mut token = Some(token);
            for &(_, func) in &self.queue {
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
    Some(
        token
            .trim_matches(|c: char| !c.is_digit(36) && c != '_')
            .into(),
    )
}

mod phf_set {
    include!(concat!(env!("OUT_DIR"), "/stop_words.rs"));
}

fn stop_word_filter(token: String) -> Option<String> {
    match phf_set::STOP_WORDS.contains(token.as_str()) {
        true => None,
        false => Some(token),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_simple_strings() {
        let string = "this is a simple string";
        assert_eq!(&tokenize(string), &["this", "is", "a", "simple", "string"]);
    }

    #[test]
    fn multiple_white_space() {
        let string = "  foo    bar  ";
        assert_eq!(&tokenize(string), &["foo", "bar"]);
    }

    #[test]
    fn hyphens() {
        let string = "take the New York-San Francisco flight";
        assert_eq!(
            &tokenize(string),
            &["take", "the", "new", "york", "san", "francisco", "flight"]
        );
    }

    #[test]
    fn splitting_strings_with_hyphens() {
        let string = "Solve for A - B";
        assert_eq!(&tokenize(string), &["solve", "for", "a", "b"]);
    }

    macro_rules! pipeline_eq {
        ($func:expr, $input:expr, $output:expr) => {
            assert_eq!(&$func($input.to_string()).unwrap(), $output);
        }
    }

    #[test]
    fn latin_characters() {
        pipeline_eq!(trimmer, "hello", "hello");
    }

    #[test]
    fn removing_punctuation() {
        pipeline_eq!(trimmer, "hello.", "hello");
        pipeline_eq!(trimmer, "it's", "it's");
        pipeline_eq!(trimmer, "james'", "james");
        pipeline_eq!(trimmer, "stop!", "stop");
        pipeline_eq!(trimmer, "first,", "first");
        pipeline_eq!(trimmer, "", "");
        pipeline_eq!(trimmer, "[tag]", "tag");
        pipeline_eq!(trimmer, "[[[tag]]]", "tag");
        pipeline_eq!(trimmer, "[[!@#@!hello]]]}}}", "hello");
        pipeline_eq!(trimmer, "~!@@@hello***()()()]]", "hello");
    }
}
