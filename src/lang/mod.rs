//! Intended to be compatible with <https://github.com/weixsong/lunr-languages>. Each supported 
//! language has a trimmer, a stop word filter, and a stemmer. Most users will not need to use 
//! these modules directly.

// Word lists originally from https://github.com/brenes/stopwords-filter
// Used under the terms of the MIT license.
// 
// Copyright (c) 2012 David J. Brenes
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

macro_rules! make_trimmer {
    ($reg:expr) => {
        pub fn trimmer(token: String) -> Option<String> {
            use ::regex::Regex;
            lazy_static! {
                static ref START: Regex = Regex::new(concat!("^[^", $reg, "]+")).unwrap();
                static ref END: Regex = Regex::new(concat!("[^", $reg, "]+$")).unwrap();
            }
            let token = START.replace(&token, "");
            Some(END.replace(&token, "").into())
        }
    };
}

macro_rules! make_stop_word_filter {
    ($words:expr) => {
        pub fn stop_word_filter(token: String) -> Option<String> {
            use ::std::collections::HashSet;
            lazy_static! {
                static ref WORDS: HashSet<&'static str> = {
                    let words = $words;
                    let mut set = HashSet::with_capacity(words.len());
                    for word in words.iter() {
                        set.insert(*word);
                    }
                    set
                };
            }
            if WORDS.contains(token.as_str()) {
                None
            } else {
                Some(token)
            }
        }
    };
}

macro_rules! make_stemmer {
    ($lang:expr) => {
        pub fn stemmer(token: String) -> Option<String> {
            use rust_stemmers::{Algorithm, Stemmer};
            lazy_static! {
                static ref STEMMER: Stemmer = Stemmer::create($lang);
            }
            Some(STEMMER.stem(&token).into())
        }
    };
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumString, ToString, EnumIter)]
pub enum Language {
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Hungarian,
    Italian,
    Portuguese,
    Romanian,
    Russian,
    Spanish,
    Swedish,
    Turkish,
    #[doc(hidden)]
    #[strum(disabled = "true")]
    __NonExhaustive,
}

impl Language {
    /// Returns the `Language` for the given two-character [ISO 639-1][iso] language code if the 
    /// language is supported. Returns `None` if not supported.
    /// 
    /// [iso]: https://en.wikipedia.org/wiki/ISO_639-1
    pub fn from_code(code: &str) -> Option<Language> {
        match code.to_ascii_lowercase().as_str() {
            "da" => Some(Language::Danish),
            "du" => Some(Language::Dutch),
            "en" => Some(Language::English),
            "fi" => Some(Language::Finnish),
            "fr" => Some(Language::French),
            "de" => Some(Language::German),
            "hu" => Some(Language::Hungarian),
            "it" => Some(Language::Italian),
            "pt" => Some(Language::Portuguese),
            "ro" => Some(Language::Romanian),
            "ru" => Some(Language::Russian),
            "es" => Some(Language::Spanish),
            "sv" => Some(Language::Swedish),
            "tr" => Some(Language::Turkish),
            _ => None,
        }
    }

    /// Returns the two-character [ISO 639-1][iso] language code for the `Language`.
    /// 
    /// [iso]: https://en.wikipedia.org/wiki/ISO_639-1
    pub fn to_code(&self) -> &'static str {
        match *self {
            Language::Danish => "da",
            Language::Dutch => "du",
            Language::English => "en",
            Language::Finnish => "fi",
            Language::French => "fr",
            Language::German => "de",
            Language::Hungarian => "hu",
            Language::Italian => "it",
            Language::Portuguese => "pt",
            Language::Romanian => "ro",
            Language::Russian => "ru",
            Language::Spanish => "es",
            Language::Swedish => "sv",
            Language::Turkish => "tr",
            _ => panic!("Don't use the __NonExhaustive variant!"),
        }
    }
}

pub mod da;
pub mod de;
pub mod du;
pub mod en;
pub mod es;
pub mod fi;
pub mod fr;
pub mod hu;
pub mod it;
pub mod pt;
pub mod ro;
pub mod ru;
pub mod sv;
pub mod tr;
