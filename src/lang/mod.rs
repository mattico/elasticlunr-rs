//! Intended to be compatible with <https://github.com/MihaiValentin/lunr-languages>. Each supported
//! language has a trimmer, a stop word filter, and a stemmer. Most users will not need to use
//! these modules directly.

use std::fmt::{self, Display};

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

macro_rules! impl_language {
    ($( ( $name:ident, $code:ident $(, #[$cfgs:meta] )? ), )+) => {
        /// Used to configure the `Index` for a specific lanugage.
        #[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Language {
            $(
                $(#[$cfgs])?
                $name,
            )+
        }

        /// A list of all the [`Language`]s enabled in the library.
        pub const LANGUAGES: &'static [Language] = &[
            $(
                $(#[$cfgs])?
                Language::$name,
            )+
        ];

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
                    $(
                        $(#[$cfgs])?
                        stringify!($code) => Some(Language::$name),
                    )+
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
                    $(
                        $(#[$cfgs])?
                        Language::$name => stringify!($code),
                    )+
                }
            }
        
            /// Creates a pipeline for the [`Language`](../lang/enum.Language.html).
            pub fn make_pipeline(&self) -> crate::pipeline::Pipeline {
                match *self {
                    $(
                        $(#[$cfgs])?
                        Language::$name => crate::lang::$code::make_pipeline(),
                    )+
                }
            }
        }

        impl Display for Language {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let name = match *self {
                    $(
                        $(#[$cfgs])?
                        Language::$name => stringify!($name),
                    )+
                };
                f.write_str(name)
            }
        }

        $(
            $(#[$cfgs])?
            pub mod $code;
        )+
    };
}

impl_language! {
    (English, en),
    (Danish, da, #[cfg(feature = "da")]),
    (Norwegian, no, #[cfg(feature = "no")]),
    (Dutch, du, #[cfg(feature = "du")]),
    (Finnish, fi, #[cfg(feature = "fi")]),
    (French, fr, #[cfg(feature = "fr")]),
    (German, de, #[cfg(feature = "de")]),
    (Italian, it, #[cfg(feature = "it")]),
    (Portuguese, pt, #[cfg(feature = "pt")]),
    (Romanian, ro, #[cfg(feature = "ro")]),
    (Russian, ru, #[cfg(feature = "ru")]),
    (Spanish, es, #[cfg(feature = "es")]),
    (Swedish, sv, #[cfg(feature = "sv")]),
    (Turkish, tr, #[cfg(feature = "tr")]),
    (Chinese, zh, #[cfg(feature = "zh")]),
    (Japanese, ja, #[cfg(feature = "ja")]),
    (Arabic, ar, #[cfg(feature = "ar")]),
}
