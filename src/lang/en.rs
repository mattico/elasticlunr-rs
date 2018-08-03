use crate::pipeline::Pipeline;
use regex::Regex;

pub fn make_pipeline() -> Pipeline {
    Pipeline {
        queue: vec![
            ("trimmer".into(), trimmer),
            ("stopWordFilter".into(), stop_word_filter),
            ("stemmer".into(), stemmer),
        ],
    }
}

pub fn trimmer(token: String) -> Option<String> {
    Some(
        token
            .trim_matches(|c: char| !c.is_digit(36) && c != '_')
            .into(),
    )
}

make_stop_word_filter!([
    "", "a", "able", "about", "across", "after", "all", "almost", "also", "am", "among", "an",
    "and", "any", "are", "as", "at", "be", "because", "been", "but", "by", "can", "cannot",
    "could", "dear", "did", "do", "does", "either", "else", "ever", "every", "for", "from", "get",
    "got", "had", "has", "have", "he", "her", "hers", "him", "his", "how", "however", "i", "if",
    "in", "into", "is", "it", "its", "just", "least", "let", "like", "likely", "may", "me",
    "might", "most", "must", "my", "neither", "no", "nor", "not", "of", "off", "often", "on",
    "only", "or", "other", "our", "own", "rather", "said", "say", "says", "she", "should", "since",
    "so", "some", "than", "that", "the", "their", "them", "then", "there", "these", "they", "this",
    "tis", "to", "too", "twas", "us", "wants", "was", "we", "were", "what", "when", "where",
    "which", "while", "who", "whom", "why", "will", "with", "would", "yet", "you", "your",
]);

static STEP_2: &[(&str, &str)] = &[
    ("ational", "ate"),
    ("tional", "tion"),
    ("enci", "ence"),
    ("anci", "ance"),
    ("izer", "ize"),
    ("bli", "ble"),
    ("alli", "al"),
    ("entli", "ent"),
    ("eli", "e"),
    ("ousli", "ous"),
    ("ization", "ize"),
    ("ation", "ate"),
    ("ator", "ate"),
    ("alism", "al"),
    ("iveness", "ive"),
    ("fulness", "ful"),
    ("ousness", "ous"),
    ("aliti", "al"),
    ("iviti", "ive"),
    ("biliti", "ble"),
    ("logi", "log"),
];

static STEP_3: &[(&str, &str)] = &[
    ("icate", "ic"),
    ("ative", ""),
    ("alize", "al"),
    ("iciti", "ic"),
    ("ical", "ic"),
    ("ful", ""),
    ("ness", ""),
];

// This is a direct port of the stemmer from elasticlunr.js
// It's not very efficient and very not-rusty, but it
// generates identical output.

struct Stemmer {
    re_mgr0: Regex,
    re_mgr1: Regex,
    re_meq1: Regex,
    re_s_v: Regex,

    re_1a: Regex,
    re2_1a: Regex,
    re_1b: Regex,
    re2_1b: Regex,
    re2_1b_2: Regex,
    re3_1b_2: Regex,
    re4_1b_2: Regex,

    re_1c: Regex,
    re_2: Regex,

    re_3: Regex,

    re_4: Regex,
    re2_4: Regex,

    re_5: Regex,
    re3_5: Regex,
}

// vowel
macro_rules! V {
    () => {
        "[aeiouy]"
    };
}

// consonant sequence
macro_rules! CS {
    () => {
        "[^aeiou][^aeiouy]*"
    };
}

// vowel sequence
macro_rules! VS {
    () => {
        "[aeiouy][aeiou]*"
    };
}

#[inline]
fn concat_string(strs: &[&str]) -> String {
    strs.iter().cloned().collect()
}

impl Stemmer {
    fn new() -> Self {
        let mgr0 = concat!("^(", CS!(), ")?", VS!(), CS!());
        let meq1 = concat!("^(", CS!(), ")?", VS!(), CS!(), "(", VS!(), ")?$");
        let mgr1 = concat!("^(", CS!(), ")?", VS!(), CS!(), VS!(), CS!());
        let s_v = concat!("^(", CS!(), ")?", V!());

        let re_mgr0 = Regex::new(&mgr0).unwrap();
        let re_mgr1 = Regex::new(&mgr1).unwrap();
        let re_meq1 = Regex::new(&meq1).unwrap();
        let re_s_v = Regex::new(&s_v).unwrap();

        let re_1a = Regex::new("^(.+?)(ss|i)es$").unwrap();
        let re2_1a = Regex::new("^(.+?)([^s])s$").unwrap();
        let re_1b = Regex::new("^(.+?)eed$").unwrap();
        let re2_1b = Regex::new("^(.+?)(ed|ing)$").unwrap();
        let re2_1b_2 = Regex::new("(at|bl|iz)$").unwrap();
        let re3_1b_2 = Regex::new("([^aeiouylsz]{2})$").unwrap();
        let re4_1b_2 = Regex::new(concat!("^", CS!(), V!(), "[^aeiouwxy]$")).unwrap();

        let re_1c = Regex::new("^(.+?[^aeiou])y$").unwrap();
        let re_2 = Regex::new(
            "^(.+?)(ational|tional|enci|anci|izer|bli|alli|entli|eli|ousli|\
             ization|ation|ator|alism|iveness|fulness|ousness|aliti|iviti|biliti|logi)$",
        ).unwrap();

        let re_3 = Regex::new("^(.+?)(icate|ative|alize|iciti|ical|ful|ness)$").unwrap();

        let re_4 = Regex::new(
            "^(.+?)(al|ance|ence|er|ic|able|ible|ant|ement|ment|ent|ou|ism|ate|iti|ous|ive|ize)$",
        ).unwrap();
        let re2_4 = Regex::new("^(.+?)(s|t)(ion)$").unwrap();

        let re_5 = Regex::new("^(.+?)e$").unwrap();
        let re3_5 = Regex::new(concat!("^", CS!(), V!(), "[^aeiouwxy]$")).unwrap();

        Stemmer {
            re_mgr0,
            re_mgr1,
            re_meq1,
            re_s_v,
            re_1a,
            re2_1a,
            re_1b,
            re2_1b,
            re2_1b_2,
            re3_1b_2,
            re4_1b_2,
            re_1c,
            re_2,
            re_3,
            re_4,
            re2_4,
            re_5,
            re3_5,
        }
    }

    /// Implements the Porter stemming algorithm
    pub fn stem(&self, mut w: String) -> String {
        if w.len() < 3 {
            return w;
        }

        let starts_with_y = w.as_bytes()[0] == b'y';
        if starts_with_y {
            w.remove(0);
            w.insert(0, 'Y');
        }

        // TODO: There's probably a better way to handle the
        // borrowchecker than cloning w a million times

        // Step 1a
        if let Some(caps) = self.re_1a.captures(&w.clone()) {
            w = concat_string(&[&caps[1], &caps[2]]);
        }
        if let Some(caps) = self.re2_1a.captures(&w.clone()) {
            w = concat_string(&[&caps[1], &caps[2]]);
        }

        // Step 1b
        if let Some(caps) = self.re_1b.captures(&w.clone()) {
            let stem = &caps[1];
            if self.re_mgr0.is_match(stem) {
                w.pop();
            }
        } else if let Some(caps) = self.re2_1b.captures(&w.clone()) {
            let stem = &caps[1];
            if self.re_s_v.is_match(&stem) {
                w = stem.into();

                let mut re3_1b_2_matched = false;

                if self.re2_1b_2.is_match(&w) {
                    w.push('e');
                } else if let Some(m) = self.re3_1b_2.find(&w.clone()) {
                    let suffix = m.as_str();
                    // Make sure the two characters are the same since we can't use backreferences
                    // these indexing operations can't panic since the regex only matches ascii characters
                    if suffix[0..1] == suffix[1..2] {
                        re3_1b_2_matched = true;
                        w.pop();
                    }
                }

                // re4_1b_2 still runs if re3_1b_2 matches but
                // the matched chcaracters are not the same
                if !re3_1b_2_matched && self.re4_1b_2.is_match(&w) {
                    w.push('e');
                }
            }
        }

        // Step 1c - replace suffix y or Y by i if preceded by a non-vowel which is not the first
        // letter of the word (so cry -> cri, by -> by, say -> say)
        if let Some(caps) = self.re_1c.captures(&w.clone()) {
            let stem = &caps[1];
            w = concat_string(&[stem, "i"]);
        }

        // Step 2
        if let Some(caps) = self.re_2.captures(&w.clone()) {
            let stem = &caps[1];
            let suffix = &caps[2];
            if self.re_mgr0.is_match(&stem) {
                w = concat_string(&[stem, STEP_2.iter().find(|&&(k, _)| k == suffix).unwrap().1]);
            }
        }

        // Step 3
        if let Some(caps) = self.re_3.captures(&w.clone()) {
            let stem = &caps[1];
            let suffix = &caps[2];
            if self.re_mgr0.is_match(&stem) {
                w = concat_string(&[stem, STEP_3.iter().find(|&&(k, _)| k == suffix).unwrap().1]);
            }
        }

        // Step 4
        if let Some(caps) = self.re_4.captures(&w.clone()) {
            let stem = &caps[1];
            if self.re_mgr1.is_match(&stem) {
                w = stem.into();
            }
        } else if let Some(caps) = self.re2_4.captures(&w.clone()) {
            let stem = concat_string(&[&caps[1], &caps[2]]);
            if self.re_mgr1.is_match(&stem) {
                w = stem;
            }
        }

        // Step 5
        if let Some(caps) = self.re_5.captures(&w.clone()) {
            let stem = &caps[1];
            if self.re_mgr1.is_match(&stem)
                || (self.re_meq1.is_match(&stem) && !(self.re3_5.is_match(&stem)))
            {
                w = stem.into();
            }
        }

        if w.ends_with("ll") && self.re_mgr1.is_match(&w) {
            w.pop();
        }

        // replace the original 'y'
        if starts_with_y {
            w.remove(0);
            w.insert(0, 'y');
        }

        w
    }
}

pub fn stemmer(input: String) -> Option<String> {
    lazy_static! {
        static ref STEMMER: Stemmer = Stemmer::new();
    }
    Some(STEMMER.stem(input))
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "bench")]
    extern crate test;
    use super::*;
    use pipeline::tokenize;

    #[test]
    fn split_simple_strings() {
        let string = "this is a simple string";
        assert_eq!(&tokenize(string), &["this", "is", "a", "simple", "string"]);
    }

    #[test]
    fn multiple_white_space() {
        let string = "  foo    bar  ";
        assert_eq!(&tokenize(string), &["foo", "bar"]);
    }

    #[test]
    fn hyphens() {
        let string = "take the New York-San Francisco flight";
        assert_eq!(
            &tokenize(string),
            &["take", "the", "new", "york", "san", "francisco", "flight"]
        );
    }

    #[test]
    fn splitting_strings_with_hyphens() {
        let string = "Solve for A - B";
        assert_eq!(&tokenize(string), &["solve", "for", "a", "b"]);
    }

    macro_rules! pipeline_eq {
        ($func:expr, $input:expr, $output:expr) => {
            assert_eq!(&$func($input.to_string()).unwrap(), $output);
        };
    }

    #[test]
    fn latin_characters() {
        pipeline_eq!(trimmer, "hello", "hello");
    }

    #[test]
    fn removing_punctuation() {
        pipeline_eq!(trimmer, "hello.", "hello");
        pipeline_eq!(trimmer, "it's", "it's");
        pipeline_eq!(trimmer, "james'", "james");
        pipeline_eq!(trimmer, "stop!", "stop");
        pipeline_eq!(trimmer, "first,", "first");
        pipeline_eq!(trimmer, "", "");
        pipeline_eq!(trimmer, "[tag]", "tag");
        pipeline_eq!(trimmer, "[[[tag]]]", "tag");
        pipeline_eq!(trimmer, "[[!@#@!hello]]]}}}", "hello");
        pipeline_eq!(trimmer, "~!@@@hello***()()()]]", "hello");
    }

    #[test]
    fn test_stemmer() {
        let cases = [
            ("consign", "consign"),
            ("consigned", "consign"),
            ("consigning", "consign"),
            ("consignment", "consign"),
            ("consist", "consist"),
            ("consisted", "consist"),
            ("consistency", "consist"),
            ("consistent", "consist"),
            ("consistently", "consist"),
            ("consisting", "consist"),
            ("consists", "consist"),
            ("consolation", "consol"),
            ("consolations", "consol"),
            ("consolatory", "consolatori"),
            ("console", "consol"),
            ("consoled", "consol"),
            ("consoles", "consol"),
            ("consolidate", "consolid"),
            ("consolidated", "consolid"),
            ("consolidating", "consolid"),
            ("consoling", "consol"),
            ("consols", "consol"),
            ("consonant", "conson"),
            ("consort", "consort"),
            ("consorted", "consort"),
            ("consorting", "consort"),
            ("conspicuous", "conspicu"),
            ("conspicuously", "conspicu"),
            ("conspiracy", "conspiraci"),
            ("conspirator", "conspir"),
            ("conspirators", "conspir"),
            ("conspire", "conspir"),
            ("conspired", "conspir"),
            ("conspiring", "conspir"),
            ("constable", "constabl"),
            ("constables", "constabl"),
            ("constance", "constanc"),
            ("constancy", "constanc"),
            ("constant", "constant"),
            ("knack", "knack"),
            ("knackeries", "knackeri"),
            ("knacks", "knack"),
            ("knag", "knag"),
            ("knave", "knave"),
            ("knaves", "knave"),
            ("knavish", "knavish"),
            ("kneaded", "knead"),
            ("kneading", "knead"),
            ("knee", "knee"),
            ("kneel", "kneel"),
            ("kneeled", "kneel"),
            ("kneeling", "kneel"),
            ("kneels", "kneel"),
            ("knees", "knee"),
            ("knell", "knell"),
            ("knelt", "knelt"),
            ("knew", "knew"),
            ("knick", "knick"),
            ("knif", "knif"),
            ("knife", "knife"),
            ("knight", "knight"),
            ("knights", "knight"),
            ("knit", "knit"),
            ("knits", "knit"),
            ("knitted", "knit"),
            ("knitting", "knit"),
            ("knives", "knive"),
            ("knob", "knob"),
            ("knobs", "knob"),
            ("knock", "knock"),
            ("knocked", "knock"),
            ("knocker", "knocker"),
            ("knockers", "knocker"),
            ("knocking", "knock"),
            ("knocks", "knock"),
            ("knopp", "knopp"),
            ("knot", "knot"),
            ("knots", "knot"),
            ("lay", "lay"),
            ("try", "tri"),
        ];

        for &(input, output) in cases.iter() {
            assert_eq!(&stemmer(input.into()).unwrap(), output);
        }
    }

    // # Results
    // PHF:           2,507,473 ns/iter (+/- 197,005)
    // Linear search: 2,481,133 ns/iter (+/- 216,998)

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_stem(b: &mut test::Bencher) {
        let text = include_str!("../../tests/data/en.in.txt");
        let tokens = ::pipeline::tokenize(text);
        b.iter(|| {
            let tokens = tokens.clone();
            for token in tokens {
                test::black_box(stemmer(token));
            }
        });
    }
}
