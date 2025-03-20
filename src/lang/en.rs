use super::{common::StopWordFilter, Language};
use crate::pipeline::{FnWrapper, Pipeline, PipelineFn};

const WORDS: &[&str] = &[
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
];

#[derive(Clone)]
pub struct English {
    stemmer: Stemmer,
}

impl English {
    pub fn new() -> Self {
        let stemmer = Stemmer::new();
        Self { stemmer }
    }
}

impl Language for English {
    fn name(&self) -> String {
        "English".into()
    }
    fn code(&self) -> String {
        "en".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(FnWrapper("trimmer".into(), trimmer)),
                Box::new(StopWordFilter::new("stopWordFilter", WORDS)),
                Box::new(self.stemmer.clone()),
            ],
        }
    }
}

fn trimmer(token: String) -> Option<String> {
    Some(
        token
            .trim_matches(|c: char| !c.is_digit(36) && c != '_')
            .into(),
    )
}

#[derive(Clone)]
struct Stemmer;

impl PipelineFn for Stemmer {
    fn name(&self) -> String {
        "stemmer".into()
    }

    fn filter(&self, token: String) -> Option<String> {
        self.stem(&token)
    }
}

impl Stemmer {
    fn new() -> Self {
        Self
    }

    fn stem(&self, word: &str) -> Option<String> {
        get(word).ok()
    }
}

/// This stemmer implementation is taken directly from rust-stem
/// (https://github.com/minhnhdo/rust-stem) which is licensed under the MIT
/// License as follows:
/// 
/// The MIT License (MIT)
///
/// Copyright (c) 2013 Do Nhat Minh

/// Member b is a vector of bytes holding a word to be stemmed.
/// The letters are in b[0], b[1] ... ending at b[z->k]. Member k is readjusted
/// downwards as the stemming progresses. Zero termination is not in fact used
/// in the algorithm.
///
/// Note that only lower case sequences are stemmed. get(...) automatically
/// lowercases the string before processing.
struct PorterStemmer {
    b: Vec<u8>,
    k: usize,
    j: usize,
}

impl PorterStemmer {
    fn new(word: &str) -> Result<PorterStemmer, &'static str> {
        let b = word.to_ascii_lowercase().into_bytes();
        let k = b.len();
        Ok(PorterStemmer { b: b, k: k, j: 0 })
    }

    /// stem.is_consonant(i) is true <=> stem[i] is a consonant
    #[inline]
    fn is_consonant(&self, i: usize) -> bool {
        match self.b[i] {
            b'a' | b'e' | b'i' | b'o' | b'u' => false,
            b'y' => {
                if i == 0 {
                    true
                } else {
                    !self.is_consonant(i - 1)
                }
            }
            _ => true,
        }
    }

    /// stem.measure() measures the number of consonant sequences in [0, j).
    /// if c is a consonant sequence and v a vowel sequence, and <..> indicates
    /// arbitrary presence,
    ///
    /// ~~~notrust
    ///    <c><v>       gives 0
    ///    <c>vc<v>     gives 1
    ///    <c>vcvc<v>   gives 2
    ///    <c>vcvcvc<v> gives 3
    ///    ....
    /// ~~~
    fn measure(&self) -> usize {
        let mut n = 0;
        let mut i = 0;
        let j = self.j;
        loop {
            if i >= j {
                return n;
            }
            if !self.is_consonant(i) {
                break;
            }
            i += 1;
        }
        i += 1;
        loop {
            loop {
                if i >= j {
                    return n;
                }
                if self.is_consonant(i) {
                    break;
                }
                i += 1;
            }
            i += 1;
            n += 1;
            loop {
                if i >= j {
                    return n;
                }
                if !self.is_consonant(i) {
                    break;
                }
                i += 1;
            }
            i += 1;
        }
    }

    /// stem.has_vowel() is TRUE <=> [0, j-1) contains a vowel
    fn has_vowel(&self) -> bool {
        for i in 0..self.j {
            if !self.is_consonant(i) {
                return true;
            }
        }
        false
    }

    /// stem.double_consonant(i) is TRUE <=> i,(i-1) contain a double consonant.
    #[inline]
    fn double_consonant(&self, i: usize) -> bool {
        if i < 1 {
            false
        } else if self.b[i] != self.b[i - 1] {
            false
        } else {
            self.is_consonant(i)
        }
    }

    /// cvc(z, i) is TRUE <=> i-2,i-1,i has the form consonant - vowel - consonant
    /// and also if the second c is not w,x or y. this is used when trying to
    /// restore an e at the end of a short word. e.g.
    ///
    /// ~~~notrust
    ///    cav(e), lov(e), hop(e), crim(e), but
    ///    snow, box, tray.
    /// ~~~
    fn cvc(&self, i: usize) -> bool {
        if i < 2 || !self.is_consonant(i) || self.is_consonant(i - 1) || !self.is_consonant(i - 2) {
            false
        } else {
            match self.b[i] {
                b'w' | b'x' | b'y' => false,
                _ => true,
            }
        }
    }

    /// stem.ends(s) is true <=> [0, k) ends with the string s.
    fn ends(&mut self, _s: &str) -> bool {
        let s = _s.as_bytes();
        let len = s.len();
        if len > self.k {
            false
        } else {
            if &self.b[self.k - len..self.k] == s {
                self.j = self.k - len;
                true
            } else {
                false
            }
        }
    }

    /// stem.setto(s) sets [j,k) to the characters in the string s,
    /// readjusting k.
    fn set_to(&mut self, s: &str) {
        let s = s.as_bytes();
        let len = s.len();
        for i in 0..(len) {
            self.b[self.j + i] = s[i];
        }
        self.k = self.j + len;
    }

    /// self.replace(s) is used further down.
    #[inline]
    fn r(&mut self, s: &str) {
        if self.measure() > 0 {
            self.set_to(s);
        }
    }

    /// stem.step1ab() gets rid of plurals and -ed or -ing. e.g.
    ///
    /// ~~~~notrust
    ///     caresses  ->  caress
    ///     ponies    ->  poni
    ///     ties      ->  ti
    ///     caress    ->  caress
    ///     cats      ->  cat
    ///
    ///     feed      ->  feed
    ///     agreed    ->  agree
    ///     disabled  ->  disable
    ///
    ///     matting   ->  mat
    ///     mating    ->  mate
    ///     meeting   ->  meet
    ///     milling   ->  mill
    ///     messing   ->  mess
    ///
    ///     meetings  ->  meet
    /// ~~~~
    fn step1ab(&mut self) {
        if self.b[self.k - 1] == b's' {
            if self.ends("sses") {
                self.k -= 2;
            } else if self.ends("ies") {
                self.set_to("i");
            } else if self.b[self.k - 2] != b's' {
                self.k -= 1;
            }
        }
        if self.ends("eed") {
            if self.measure() > 0 {
                self.k -= 1
            }
        } else if (self.ends("ed") || self.ends("ing")) && self.has_vowel() {
            self.k = self.j;
            if self.ends("at") {
                self.set_to("ate");
            } else if self.ends("bl") {
                self.set_to("ble");
            } else if self.ends("iz") {
                self.set_to("ize");
            } else if self.double_consonant(self.k - 1) {
                self.k -= 1;
                match self.b[self.k - 1] {
                    b'l' | b's' | b'z' => self.k += 1,
                    _ => (),
                }
            } else if self.measure() == 1 && self.cvc(self.k - 1) {
                self.set_to("e");
            }
        }
    }

    /// stem.step1c() turns terminal y to i when there is another vowel in the stem.
    fn step1c(&mut self) {
        if self.ends("y") && self.is_consonant(self.k - 2) && self.k > 2 {
            self.b[self.k - 1] = b'i';
        }
    }

    /// stem.step2() maps double suffices to single ones. so -ization ( = -ize
    /// plus -ation) maps to -ize etc. note that the string before the suffix
    /// must give m(z) > 0.
    fn step2(&mut self) {
        if self.k < 2 {
            return;
        }
        match self.b[self.k - 2] {
            b'a' => {
                if self.ends("ational") {
                    self.r("ate");
                    return;
                }
                if self.ends("tional") {
                    self.r("tion");
                    return;
                }
            }
            b'c' => {
                if self.ends("enci") {
                    self.r("ence");
                    return;
                }
                if self.ends("anci") {
                    self.r("ance");
                    return;
                }
            }
            b'e' => {
                if self.ends("izer") {
                    self.r("ize");
                    return;
                }
            }
            b'l' => {
                if self.ends("bli") {
                    self.r("ble");
                    return;
                } /*-DEPARTURE-*/

                /* To match the published algorithm, replace this line with
                'l' => {
                    if self.ends("abli") { self.r("able"); return } */

                if self.ends("alli") {
                    self.r("al");
                    return;
                }
                if self.ends("entli") {
                    self.r("ent");
                    return;
                }
                if self.ends("eli") {
                    self.r("e");
                    return;
                }
                if self.ends("ousli") {
                    self.r("ous");
                    return;
                }
            }
            b'o' => {
                if self.ends("ization") {
                    self.r("ize");
                    return;
                }
                if self.ends("ation") {
                    self.r("ate");
                    return;
                }
                if self.ends("ator") {
                    self.r("ate");
                    return;
                }
            }
            b's' => {
                if self.ends("alism") {
                    self.r("al");
                    return;
                }
                if self.ends("iveness") {
                    self.r("ive");
                    return;
                }
                if self.ends("fulness") {
                    self.r("ful");
                    return;
                }
                if self.ends("ousness") {
                    self.r("ous");
                    return;
                }
            }
            b't' => {
                if self.ends("aliti") {
                    self.r("al");
                    return;
                }
                if self.ends("iviti") {
                    self.r("ive");
                    return;
                }
                if self.ends("biliti") {
                    self.r("ble");
                    return;
                }
            }
            b'g' => {
                if self.ends("logi") {
                    self.r("log");
                    return;
                }
            } /*-DEPARTURE-*/
            /* To match the published algorithm, delete this line */
            _ => (),
        }
    }

    /// stem.step3() deals with -ic-, -full, -ness etc. similar strategy to step2.
    fn step3(&mut self) {
        match self.b[self.k - 1] {
            b'e' => {
                if self.ends("icate") {
                    self.r("ic");
                    return;
                }
                if self.ends("ative") {
                    self.r("");
                    return;
                }
                if self.ends("alize") {
                    self.r("al");
                    return;
                }
            }
            b'i' => {
                if self.ends("iciti") {
                    self.r("ic");
                    return;
                }
            }
            b'l' => {
                if self.ends("ical") {
                    self.r("ic");
                    return;
                }
                if self.ends("ful") {
                    self.r("");
                    return;
                }
            }
            b's' => {
                if self.ends("ness") {
                    self.r("");
                    return;
                }
            }
            _ => (),
        }
    }

    /// stem.step4() takes off -ant, -ence etc., in context <c>vcvc<v>.
    fn step4(&mut self) {
        if self.k < 2 {
            return;
        }
        match self.b[self.k - 2] {
            b'a' => {
                if self.ends("al") {
                } else {
                    return;
                }
            }
            b'c' => {
                if self.ends("ance") {
                } else if self.ends("ence") {
                } else {
                    return;
                }
            }
            b'e' => {
                if self.ends("er") {
                } else {
                    return;
                }
            }
            b'i' => {
                if self.ends("ic") {
                } else {
                    return;
                }
            }
            b'l' => {
                if self.ends("able") {
                } else if self.ends("ible") {
                } else {
                    return;
                }
            }
            b'n' => {
                if self.ends("ant") {
                } else if self.ends("ement") {
                } else if self.ends("ment") {
                } else if self.ends("ent") {
                } else {
                    return;
                }
            }
            b'o' => {
                if self.ends("ion") && self.j > 0 && (self.b[self.j - 1] == b's' || self.b[self.j - 1] == b't') {
                } else if self.ends("ou") {
                } else {
                    return;
                }
                /* takes care of -ous */
            }
            b's' => {
                if self.ends("ism") {
                } else {
                    return;
                }
            }
            b't' => {
                if self.ends("ate") {
                } else if self.ends("iti") {
                } else {
                    return;
                }
            }
            b'u' => {
                if self.ends("ous") {
                } else {
                    return;
                }
            }
            b'v' => {
                if self.ends("ive") {
                } else {
                    return;
                }
            }
            b'z' => {
                if self.ends("ize") {
                } else {
                    return;
                }
            }
            _ => return,
        }
        if self.measure() > 1 {
            self.k = self.j
        }
    }

    /// stem.step5() removes a final -e if self.measure() > 0, and changes -ll
    /// to -l if self.measure() > 1.
    fn step5(&mut self) {
        self.j = self.k;
        if self.b[self.k - 1] == b'e' {
            let a = self.measure();
            if a > 1 || a == 1 && !self.cvc(self.k - 2) {
                self.k -= 1
            }
        }
        if self.b[self.k - 1] == b'l' && self.double_consonant(self.k - 1) && self.measure() > 1 {
            self.k -= 1;
        }
    }

    #[inline]
    fn get(&self) -> String {
        String::from_utf8(self.b[..self.k].to_vec()).unwrap()
    }
}

fn get(word: &str) -> Result<String, &str> {
    if word.len() > 2 {
        match PorterStemmer::new(word) {
            Ok(w) => {
                let mut mw = w;
                mw.step1ab();
                mw.step1c();
                mw.step2();
                mw.step3();
                mw.step4();
                mw.step5();
                Ok(mw.get())
            }
            Err(e) => Err(e),
        }
    } else {
        Ok(word.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            ("activate", "activ"),
            ("adjustable", "adjust"),
            ("adjustment", "adjust"),
            ("adoption", "adopt"),
            ("airliner", "airlin"),
            ("allowance", "allow"),
            ("analogousli", "analog"),
            ("angulariti", "angular"),
            ("bleeding", "bleed"),
            ("bowdlerize", "bowdler"),
            ("by", "by"),
            ("callousness", "callous"),
            ("caresses", "caress"),
            ("communism", "commun"),
            ("conditional", "condit"),
            ("conformabli", "conform"),
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
            ("controll", "control"),
            ("decisiveness", "decis"),
            ("defensible", "defens"),
            ("dependent", "depend"),
            ("differentli", "differ"),
            ("digitizer", "digit"),
            ("effective", "effect"),
            ("eing", "e"),
            ("electrical", "electr"),
            ("electriciti", "electr"),
            ("feudalism", "feudal"),
            ("formaliti", "formal"),
            ("formalize", "formal"),
            ("formative", "form"),
            ("goodness", "good"),
            ("gyroscopic", "gyroscop"),
            ("hesitanci", "hesit"),
            ("homologou", "homolog"),
            ("homologous", "homolog"),
            ("hopeful", "hope"),
            ("hopefulness", "hope"),
            ("inference", "infer"),
            ("ion", "ion"),
            ("irritant", "irrit"),
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
            ("operator", "oper"),
            ("predication", "predic"),
            ("radicalli", "radic"),
            ("rational", "ration"),
            ("relational", "relat"),
            ("replacement", "replac"),
            ("revival", "reviv"),
            ("roll", "roll"),
            ("sensibiliti", "sensibl"),
            ("sensitiviti", "sensit"),
            ("triplicate", "triplic"),
            ("try", "tri"),
            ("valenci", "valenc"),
            ("vietnamization", "vietnam"),
            ("vileli", "vile"),
            ("young", "young"),
        ];

        let stemmer = Stemmer::new();
        for &(input, output) in cases.iter() {
            let result = stemmer.stem(input.into()).unwrap();
            assert_eq!(&result, output);
        }
    }
}
