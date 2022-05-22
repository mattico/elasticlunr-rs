use super::Language;
use crate::pipeline::{FnWrapper, Pipeline};

pub struct Chinese {
    jieba: jieba_rs::Jieba,
}

impl Chinese {
    pub fn new() -> Self {
        Self {
            jieba: jieba_rs::Jieba::new(),
        }
    }
}

impl Language for Chinese {
    fn name(&self) -> String {
        "Chinese".into()
    }
    fn code(&self) -> String {
        "zh".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        self.jieba
            .cut_for_search(text, false)
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(FnWrapper("trimmer-zh".into(), trimmer)),
                Box::new(FnWrapper("stopWordFilter-zh".into(), stop_word_filter)),
                Box::new(FnWrapper("stemmer-zh".into(), stemmer)),
            ],
        }
    }
}

pub fn trimmer(token: String) -> Option<String> {
    let ret: String = token.trim_matches(|c: char| !is_valid_char(c)).into();

    if ret.eq("") {
        return None;
    }

    Some(ret)
}

fn stop_word_filter(token: String) -> Option<String> {
    match token.as_str() {
        "的" | "了" => None,
        _ => Some(token),
    }
}

fn stemmer(token: String) -> Option<String> {
    Some(token)
}

fn is_valid_char(c: char) -> bool {
    let min_max_list = [
        [19668, 40869], // min and max Chinese char
        ['a' as u32, 'z' as u32],
        ['A' as u32, 'Z' as u32],
    ];

    let c = c as u32;
    for min_max in min_max_list.iter() {
        if c >= min_max[0] && c <= min_max[1] {
            return true;
        }
    }

    false
}
