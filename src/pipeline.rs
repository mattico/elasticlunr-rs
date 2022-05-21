//! Defines the pipeline which processes text for inclusion in the index. Most users do not need
//! to use this module directly.

use serde::ser::{Serialize, SerializeSeq, Serializer};

/// Splits a text string into a vector of individual tokens.
pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| c.is_whitespace() || c == '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_lowercase())
        .collect()
}

#[cfg(feature = "zh")]
pub fn tokenize_chinese(text: &str) -> Vec<String> {
    let jieba = jieba_rs::Jieba::new();

    jieba
        .cut_for_search(text.as_ref(), false)
        .iter()
        .map(|s| (*s).into())
        .collect()
}

#[cfg(feature = "ja")]
pub fn tokenize_japanese(text: &str) -> Vec<String> {
    use lindera::tokenizer::{Tokenizer, TokenizerConfig};
    use lindera_core::viterbi::Mode;
    let config = TokenizerConfig {
        mode: Mode::Decompose(Default::default()),
        ..Default::default()
    };
    // NB: unwrap() is okay since the errors are only related to user-supplied dictionaries.
    let tokenizer = Tokenizer::with_config(config).unwrap();
    tokenizer
        .tokenize(text)
        .unwrap()
        .into_iter()
        .filter_map(|tok| match tok.detail.get(0).map(|d| d.as_str()) {
            Some("助詞") | Some("助動詞") | Some("記号") | Some("UNK") => None,
            _ => Some(tok.text.to_string()),
        })
        .collect()
}

/// The function type used for the tokenizer.
pub type TokenizerFn = fn(&str) -> Vec<String>;

/// The function type used for each step in a pipeline.
pub type PipelineFn = fn(String) -> Option<String>;

/// A sequence of `PipelineFn`s which are run on tokens to prepare them for searching.
#[derive(Debug, Deserialize)]
pub struct Pipeline {
    #[serde(skip_deserializing)]
    pub queue: Vec<(String, PipelineFn)>,
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
        crate::lang::en::make_pipeline()
    }
}

impl Pipeline {
    /// Returns a pipeline for the given [`Language`](../lang/enum.Language.html).
    #[deprecated(since = "2.2.0", note = "Use `Language::make_pipeline()`")]
    pub fn for_language(lang: crate::lang::Language) -> Pipeline {
        lang.make_pipeline()
    }

    /// Run the Pipeline against the given vector of tokens. The returned vector may be shorter
    /// than the input if a pipeline function returns `None` for a token.
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
