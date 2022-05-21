//! Defines the pipeline which processes text for inclusion in the index. Most users do not need
//! to use this module directly.

use serde::ser::{Serialize, SerializeSeq, Serializer};

/// The function type used for the tokenizer.
pub type TokenizerFn = fn(&str) -> Vec<String>;

pub trait PipelineFn {
    fn name(&self) -> String;

    fn filter(&mut self, token: String) -> Option<String>;
}

impl PipelineFn for (String, dyn Fn(String) -> Option<String>) {
    fn name(&self) -> String {
        self.0.clone()
    }

    fn filter(&mut self, token: String) -> Option<String> {
        (self.1)(token)
    }
}

/// A sequence of `PipelineFn`s which are run on tokens to prepare them for searching.
#[derive(Debug, Deserialize)]
pub struct Pipeline {
    #[serde(skip_deserializing)]
    pub queue: Vec<dyn PipelineFn>,
}

impl Serialize for Pipeline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.queue.len()))?;
        for &elem in &self.queue {
            seq.serialize_element(&elem.name())?;
        }
        seq.end()
    }
}

impl Pipeline {
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
