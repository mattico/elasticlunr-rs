//! Defines the pipeline which processes text for inclusion in the index. Most users do not need
//! to use this module directly.

use serde::ser::{Serialize, Serializer, SerializeSeq};

/// Splits a text string into a vector of individual tokens.
pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| c.is_whitespace() || c == '-')
        .filter(|s| s.len() > 0)
        .map(|s| s.trim().to_lowercase())
        .collect()
}

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
        ::lang::en::make_pipeline()
    }
}

impl Pipeline {
    /// Returns a pipeline for the given [`Language`](../lang/enum.Language.html).
    #[deprecated(since = "2.2.0", note = "Use `Language::make_pipeline()`")]
    pub fn for_language(lang: ::lang::Language) -> Pipeline {
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
