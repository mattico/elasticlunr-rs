use super::Language;
use crate::pipeline::Pipeline;
use lindera::tokenizer::{Tokenizer, TokenizerConfig};
use lindera_core::viterbi::Mode;

pub struct Japanese {
    tokenizer: Tokenizer,
}

impl Japanese {
    pub fn new() -> Self {
        let config = TokenizerConfig {
            mode: Mode::Decompose(Default::default()),
            ..Default::default()
        };
        Self::with_config(config)
    }

    pub fn with_config(config: TokenizerConfig) -> Self {
        // NB: unwrap() is okay since the errors are only related to user-supplied dictionaries.
        let tokenizer = Tokenizer::with_config(config).unwrap();
        Self { tokenizer }
    }
}

impl Language for Japanese {
    fn name(&self) -> String {
        "Japanese".into()
    }
    fn code(&self) -> String {
        "ja".into()
    }

    fn tokenize(&mut self, text: &str) -> Vec<String> {
        self.tokenizer
            .tokenize(text)
            .unwrap()
            .into_iter()
            .filter_map(|tok| match tok.detail.get(0).map(|d| d.as_str()) {
                Some("助詞") | Some("助動詞") | Some("記号") | Some("UNK") => None,
                _ => Some(tok.text.to_string()),
            })
            .collect()
    }

    fn pipeline(&mut self) -> Pipeline {
        Pipeline {
            queue: vec![
                ("trimmer-ja".into(), trimmer),
                ("stemmer-ja".into(), stemmer),
            ],
        }
    }
}

pub fn trimmer(token: String) -> Option<String> {
    match token.trim_matches(|c| !is_valid_char(c)) {
        x if x.is_empty() => None,
        x => Some(x.to_string()),
    }
}

fn stemmer(token: String) -> Option<String> {
    Some(token)
}

fn is_valid_char(c: char) -> bool {
    let min_max_list = [
        ('0' as u32, '9' as u32),
        ('a' as u32, 'z' as u32),
        ('A' as u32, 'Z' as u32),
        // the following ranges are where Japanese characters are placed
        ('\u{3041}' as u32, '\u{30A0}' as u32),
        ('\u{30A0}' as u32, '\u{30FF}' as u32),
        ('\u{31F0}' as u32, '\u{31FF}' as u32),
        ('\u{3099}' as u32, '\u{309C}' as u32),
        ('\u{4E00}' as u32, '\u{9FCf}' as u32),
        ('\u{F900}' as u32, '\u{FAFF}' as u32),
        ('\u{3400}' as u32, '\u{4DBF}' as u32),
    ];
    let c = c as u32;
    min_max_list.iter().any(|&(min, max)| min <= c && c <= max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_char() {
        assert!(is_valid_char('0'));
        assert!(is_valid_char('9'));
        assert!(is_valid_char('a'));
        assert!(is_valid_char('z'));
        assert!(is_valid_char('A'));
        assert!(is_valid_char('Z'));
        assert!(is_valid_char('あ'));
        assert!(is_valid_char('ん'));
        assert!(is_valid_char('ア'));
        assert!(is_valid_char('ン'));
        assert!(is_valid_char('亜'));
        assert!(is_valid_char('日'));
        assert!(is_valid_char('本'));
        assert!(is_valid_char('語'));
        assert!(is_valid_char('ー'));
        assert!(!is_valid_char('。'));
        assert!(!is_valid_char('！'));
        assert!(!is_valid_char('〜'));
    }

    #[test]
    fn test_trimmer() {
        assert_eq!(
            trimmer("  こんにちは、世界！".to_string()),
            Some("こんにちは、世界".to_string())
        );
    }
}
