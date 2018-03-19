use serde::ser::{Serialize, Serializer, SerializeSeq};
use lang::Language;

/// Splits a text string into a vector of individual tokens.
pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| c.is_whitespace() || c == '-')
        .filter(|s| s.len() > 0)
        .map(|s| String::from(s.to_ascii_lowercase()))
        .collect()
}

/// The function type used for each step in a pipeline.
pub type PipelineFn = fn(String) -> Option<String>;

/// A sequence of `PipelineFn`s which are run on tokens to prepare them for searching.
#[derive(Debug)]
pub struct Pipeline {
    pub(crate) queue: Vec<(String, PipelineFn)>,
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
    pub fn for_language(lang: Language) -> Pipeline {
        match lang {
            Language::Danish => ::lang::da::make_pipeline(),
            Language::Dutch => ::lang::du::make_pipeline(),
            Language::English => ::lang::en::make_pipeline(),
            Language::Finnish => ::lang::fi::make_pipeline(),
            Language::French => ::lang::fr::make_pipeline(),
            Language::German => ::lang::de::make_pipeline(),
            Language::Hungarian => ::lang::hu::make_pipeline(),
            Language::Italian => ::lang::it::make_pipeline(),
            Language::Portuguese => ::lang::pt::make_pipeline(),
            Language::Romanian => ::lang::ro::make_pipeline(),
            Language::Russian => ::lang::ru::make_pipeline(),
            Language::Spanish => ::lang::es::make_pipeline(),
            Language::Swedish => ::lang::sv::make_pipeline(),
            Language::Turkish => ::lang::tr::make_pipeline(),
            _ => panic!("Dont use the `__NonExhaustive` variant!"),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_pipeline() {
        for lang in ::lang::SUPPORTED_LANGUAGES {
            assert!(Pipeline::for_language(lang).is_some());
        }
        for lang in &["jp", "zh", "kr", "tw", "tu"] {
            assert!(Pipeline::for_language(lang).is_none());
        }
    }
}
