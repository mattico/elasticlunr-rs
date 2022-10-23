use super::{common::RegexTrimmer, Language};
use crate::pipeline::{FnWrapper, Pipeline};

#[derive(Clone)]
pub struct Korean {
}

impl Korean {
    pub fn new() -> Self {
        Self { }
    }
}

impl Language for Korean {
    fn name(&self) -> String {
        "Korean".into()
    }
    fn code(&self) -> String {
        "ko".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(RegexTrimmer::new("trimmer-ko", r"0-9A-Za-z\p{Hangul}")),
                Box::new(FnWrapper("stemmer-ko".into(), stemmer)),
            ],
        }
    }
}

fn stemmer(token: String) -> Option<String> {
    Some(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            Korean::new().tokenize(" 한글 사랑 "),
            vec!["한글", "사랑"]
        );
    }

    #[test]
    fn test_pipeline() {
        let p = Korean::new().make_pipeline();
        let v = vec![" 한글 사랑!".into()];

        assert_eq!(
            p.run(v),
            vec!["한글 사랑".to_string()]
        );
    }
}
