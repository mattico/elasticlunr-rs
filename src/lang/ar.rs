/// Simple Arabic stemmer based on lunr.ar.js from https://github.com/MihaiValentin/lunr-languages

use crate::pipeline::Pipeline;
use regex::Regex;

pub fn make_pipeline() -> Pipeline {
    Pipeline {
        queue: vec![
            ("stemmer-ar".into(), stemmer),
        ],
    }
}

fn stemmer(token: String) -> Option<String> {
    lazy_static! {
        static ref DIACRITICS: Regex = Regex::new("[\u{064b}-\u{065b}]").unwrap();
        static ref ALEFS: Regex = Regex::new("[\u{0622}\u{0623}\u{0625}\u{0671}\u{0649}]").unwrap();
    }
    // remove elongating character
    let token = token.replace('\u{0640}', "");
    // remove diacritics
    let token = DIACRITICS.replace(&token, "");
    // replace all variations of alef (آأإٱى) to a plain alef (ا)
    let token = ALEFS.replace(&token, "\u{0627}");

    Some(token.into())
}
