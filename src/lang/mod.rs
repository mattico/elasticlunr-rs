//! Intended to be compatible with <https://github.com/MihaiValentin/lunr-languages>. Each supported
//! language has a trimmer, a stop word filter, and a stemmer. Most users will not need to use
//! these modules directly.

#[allow(unused_macros)]
macro_rules! make_trimmer {
    ($reg:expr) => {
        pub fn trimmer(token: String) -> Option<String> {
            use regex::Regex;
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
            use std::collections::HashSet;
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

#[cfg(feature = "rust-stemmers")]
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

/// Used to configure the `Index` for a specific lanugage.
#[derive(
    Copy, Clone, Eq, PartialEq, Debug, EnumString, ToString, EnumIter, Serialize, Deserialize,
)]
pub enum Language {
    English,
    #[cfg(feature = "da")]
    Danish,
    #[cfg(feature = "no")]
    Norwegian,
    #[cfg(feature = "du")]
    Dutch,
    #[cfg(feature = "fi")]
    Finnish,
    #[cfg(feature = "fr")]
    French,
    #[cfg(feature = "de")]
    German,
    #[cfg(feature = "it")]
    Italian,
    #[cfg(feature = "pt")]
    Portuguese,
    #[cfg(feature = "ro")]
    Romanian,
    #[cfg(feature = "ru")]
    Russian,
    #[cfg(feature = "es")]
    Spanish,
    #[cfg(feature = "sv")]
    Swedish,
    #[cfg(feature = "tr")]
    Turkish,
    #[cfg(feature = "zh")]
    Chinese,
    #[cfg(feature = "ja")]
    Japanese,
    #[doc(hidden)]
    #[strum(disabled)]
    __NonExhaustive,
}

impl Language {
    /// Returns the `Language` for the given two-character [ISO 639-1][iso] language code if the
    /// language is supported. Returns `None` if not supported.
    ///
    /// *Note:*
    ///
    /// The ISO 639-1 code for Dutch is "nl". However "du" is used for the module name
    /// and pipeline suffix in order to match lunr-languages.
    ///
    /// [iso]: https://en.wikipedia.org/wiki/ISO_639-1
    pub fn from_code(code: &str) -> Option<Language> {
        match code.to_ascii_lowercase().as_str() {
            "en" => Some(Language::English),
            #[cfg(feature = "da")]
            "da" => Some(Language::Danish),
            #[cfg(feature = "no")]
            "no" => Some(Language::Norwegian),
            #[cfg(feature = "du")]
            "nl" => Some(Language::Dutch),
            #[cfg(feature = "fi")]
            "fi" => Some(Language::Finnish),
            #[cfg(feature = "fr")]
            "fr" => Some(Language::French),
            #[cfg(feature = "de")]
            "de" => Some(Language::German),
            #[cfg(feature = "it")]
            "it" => Some(Language::Italian),
            #[cfg(feature = "pt")]
            "pt" => Some(Language::Portuguese),
            #[cfg(feature = "ro")]
            "ro" => Some(Language::Romanian),
            #[cfg(feature = "ru")]
            "ru" => Some(Language::Russian),
            #[cfg(feature = "es")]
            "es" => Some(Language::Spanish),
            #[cfg(feature = "sv")]
            "sv" => Some(Language::Swedish),
            #[cfg(feature = "tr")]
            "tr" => Some(Language::Turkish),
            #[cfg(feature = "zh")]
            "zh" => Some(Language::Chinese),
            #[cfg(feature = "ja")]
            "ja" => Some(Language::Japanese),
            _ => None,
        }
    }

    /// Returns the two-character [ISO 639-1][iso] language code for the `Language`.
    ///
    /// *Note:*
    ///
    /// The ISO 639-1 code for Dutch is "nl". However "du" is used for the module name
    /// and pipeline suffix in order to match lunr-languages.
    ///
    /// [iso]: https://en.wikipedia.org/wiki/ISO_639-1
    pub fn to_code(&self) -> &'static str {
        match *self {
            Language::English => "en",
            #[cfg(feature = "da")]
            Language::Danish => "da",
            #[cfg(feature = "no")]
            Language::Norwegian => "no",
            #[cfg(feature = "du")]
            Language::Dutch => "nl",
            #[cfg(feature = "fi")]
            Language::Finnish => "fi",
            #[cfg(feature = "fr")]
            Language::French => "fr",
            #[cfg(feature = "de")]
            Language::German => "de",
            #[cfg(feature = "it")]
            Language::Italian => "it",
            #[cfg(feature = "pt")]
            Language::Portuguese => "pt",
            #[cfg(feature = "ro")]
            Language::Romanian => "ro",
            #[cfg(feature = "ru")]
            Language::Russian => "ru",
            #[cfg(feature = "es")]
            Language::Spanish => "es",
            #[cfg(feature = "sv")]
            Language::Swedish => "sv",
            #[cfg(feature = "tr")]
            Language::Turkish => "tr",
            #[cfg(feature = "zh")]
            Language::Chinese => "zh",
            #[cfg(feature = "ja")]
            Language::Japanese => "ja",
            _ => panic!("Don't use the __NonExhaustive variant!"),
        }
    }

    /// Creates a pipeline for the [`Language`](../lang/enum.Language.html).
    pub fn make_pipeline(&self) -> crate::pipeline::Pipeline {
        match *self {
            Language::English => crate::lang::en::make_pipeline(),
            #[cfg(feature = "da")]
            Language::Danish => crate::lang::da::make_pipeline(),
            #[cfg(feature = "no")]
            Language::Norwegian => crate::lang::no::make_pipeline(),
            #[cfg(feature = "du")]
            Language::Dutch => crate::lang::du::make_pipeline(),
            #[cfg(feature = "fi")]
            Language::Finnish => crate::lang::fi::make_pipeline(),
            #[cfg(feature = "fr")]
            Language::French => crate::lang::fr::make_pipeline(),
            #[cfg(feature = "de")]
            Language::German => crate::lang::de::make_pipeline(),
            #[cfg(feature = "it")]
            Language::Italian => crate::lang::it::make_pipeline(),
            #[cfg(feature = "pt")]
            Language::Portuguese => crate::lang::pt::make_pipeline(),
            #[cfg(feature = "ro")]
            Language::Romanian => crate::lang::ro::make_pipeline(),
            #[cfg(feature = "ru")]
            Language::Russian => crate::lang::ru::make_pipeline(),
            #[cfg(feature = "es")]
            Language::Spanish => crate::lang::es::make_pipeline(),
            #[cfg(feature = "sv")]
            Language::Swedish => crate::lang::sv::make_pipeline(),
            #[cfg(feature = "tr")]
            Language::Turkish => crate::lang::tr::make_pipeline(),
            #[cfg(feature = "zh")]
            Language::Chinese => crate::lang::zh::make_pipeline(),
            #[cfg(feature = "ja")]
            Language::Japanese => crate::lang::ja::make_pipeline(),
            _ => panic!("Dont use the `__NonExhaustive` variant!"),
        }
    }
}

pub mod en;

#[cfg(feature = "da")]
pub mod da;
#[cfg(feature = "de")]
pub mod de;
#[cfg(feature = "du")]
pub mod du;
#[cfg(feature = "es")]
pub mod es;
#[cfg(feature = "fi")]
pub mod fi;
#[cfg(feature = "fr")]
pub mod fr;
#[cfg(feature = "it")]
pub mod it;
#[cfg(feature = "ja")]
pub mod ja;
#[cfg(feature = "no")]
pub mod no;
#[cfg(feature = "pt")]
pub mod pt;
#[cfg(feature = "ro")]
pub mod ro;
#[cfg(feature = "ru")]
pub mod ru;
#[cfg(feature = "sv")]
pub mod sv;
#[cfg(feature = "tr")]
pub mod tr;
#[cfg(feature = "zh")]
pub mod zh;
