use serde::ser::{Serialize, Serializer, SerializeSeq};

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
    /// Returns a pipeline for the given [ISO 639-1][iso] language code if the language is supported.
    /// Returns `None` if the language is not supported. See 
    /// [`SUPPORTED_LANGUAGES`](../lang/static.SUPPORTED_LANGUAGES.html) for a list of the supported languages.
    /// 
    /// [iso]: https://en.wikipedia.org/wiki/ISO_639-1
    pub fn for_language(lang: &str) -> Option<Pipeline> {
        match lang {
            "da" => Some(::lang::da::make_pipeline()),
            "de" => Some(::lang::de::make_pipeline()),
            "du" => Some(::lang::du::make_pipeline()),
            "en" => Some(::lang::en::make_pipeline()),
            "es" => Some(::lang::es::make_pipeline()),
            "fi" => Some(::lang::fi::make_pipeline()),
            "fr" => Some(::lang::fr::make_pipeline()),
            "hu" => Some(::lang::hu::make_pipeline()),
            "it" => Some(::lang::it::make_pipeline()),
            "pt" => Some(::lang::pt::make_pipeline()),
            "ro" => Some(::lang::ro::make_pipeline()),
            "ru" => Some(::lang::ru::make_pipeline()),
            "sv" => Some(::lang::sv::make_pipeline()),
            "tr" => Some(::lang::tr::make_pipeline()),
            _ => None,
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
