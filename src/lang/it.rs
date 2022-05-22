use super::{
    common::{RustStemmer, StopWordFilter, Trimmer},
    Language,
};
use crate::pipeline::Pipeline;
use rust_stemmers::Algorithm;

pub struct Italian {}

impl Italian {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for Italian {
    fn name(&self) -> String {
        "Italian".into()
    }
    fn code(&self) -> String {
        "it".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(Trimmer::new("trimmer-it", r"\p{Latin}")),
                Box::new(StopWordFilter::new("stopWordFilter-it", STOP_WORDS)),
                Box::new(RustStemmer::new("stemmer-it", Algorithm::Italian)),
            ],
        }
    }
}

const STOP_WORDS: &'static [&'static str] = &[
    "",
    "a",
    "abbia",
    "abbiamo",
    "abbiano",
    "abbiate",
    "ad",
    "agl",
    "agli",
    "ai",
    "al",
    "all",
    "alla",
    "alle",
    "allo",
    "anche",
    "avemmo",
    "avendo",
    "avesse",
    "avessero",
    "avessi",
    "avessimo",
    "aveste",
    "avesti",
    "avete",
    "aveva",
    "avevamo",
    "avevano",
    "avevate",
    "avevi",
    "avevo",
    "avrai",
    "avranno",
    "avrebbe",
    "avrebbero",
    "avrei",
    "avremmo",
    "avremo",
    "avreste",
    "avresti",
    "avrete",
    "avrà",
    "avrò",
    "avuta",
    "avute",
    "avuti",
    "avuto",
    "c",
    "che",
    "chi",
    "ci",
    "coi",
    "col",
    "come",
    "con",
    "contro",
    "cui",
    "da",
    "dagl",
    "dagli",
    "dai",
    "dal",
    "dall",
    "dalla",
    "dalle",
    "dallo",
    "degl",
    "degli",
    "dei",
    "del",
    "dell",
    "della",
    "delle",
    "dello",
    "di",
    "dov",
    "dove",
    "e",
    "ebbe",
    "ebbero",
    "ebbi",
    "ed",
    "era",
    "erano",
    "eravamo",
    "eravate",
    "eri",
    "ero",
    "essendo",
    "faccia",
    "facciamo",
    "facciano",
    "facciate",
    "faccio",
    "facemmo",
    "facendo",
    "facesse",
    "facessero",
    "facessi",
    "facessimo",
    "faceste",
    "facesti",
    "faceva",
    "facevamo",
    "facevano",
    "facevate",
    "facevi",
    "facevo",
    "fai",
    "fanno",
    "farai",
    "faranno",
    "farebbe",
    "farebbero",
    "farei",
    "faremmo",
    "faremo",
    "fareste",
    "faresti",
    "farete",
    "farà",
    "farò",
    "fece",
    "fecero",
    "feci",
    "fosse",
    "fossero",
    "fossi",
    "fossimo",
    "foste",
    "fosti",
    "fu",
    "fui",
    "fummo",
    "furono",
    "gli",
    "ha",
    "hai",
    "hanno",
    "ho",
    "i",
    "il",
    "in",
    "io",
    "l",
    "la",
    "le",
    "lei",
    "li",
    "lo",
    "loro",
    "lui",
    "ma",
    "mi",
    "mia",
    "mie",
    "miei",
    "mio",
    "ne",
    "negl",
    "negli",
    "nei",
    "nel",
    "nell",
    "nella",
    "nelle",
    "nello",
    "noi",
    "non",
    "nostra",
    "nostre",
    "nostri",
    "nostro",
    "o",
    "per",
    "perché",
    "più",
    "quale",
    "quanta",
    "quante",
    "quanti",
    "quanto",
    "quella",
    "quelle",
    "quelli",
    "quello",
    "questa",
    "queste",
    "questi",
    "questo",
    "sarai",
    "saranno",
    "sarebbe",
    "sarebbero",
    "sarei",
    "saremmo",
    "saremo",
    "sareste",
    "saresti",
    "sarete",
    "sarà",
    "sarò",
    "se",
    "sei",
    "si",
    "sia",
    "siamo",
    "siano",
    "siate",
    "siete",
    "sono",
    "sta",
    "stai",
    "stando",
    "stanno",
    "starai",
    "staranno",
    "starebbe",
    "starebbero",
    "starei",
    "staremmo",
    "staremo",
    "stareste",
    "staresti",
    "starete",
    "starà",
    "starò",
    "stava",
    "stavamo",
    "stavano",
    "stavate",
    "stavi",
    "stavo",
    "stemmo",
    "stesse",
    "stessero",
    "stessi",
    "stessimo",
    "steste",
    "stesti",
    "stette",
    "stettero",
    "stetti",
    "stia",
    "stiamo",
    "stiano",
    "stiate",
    "sto",
    "su",
    "sua",
    "sue",
    "sugl",
    "sugli",
    "sui",
    "sul",
    "sull",
    "sulla",
    "sulle",
    "sullo",
    "suo",
    "suoi",
    "ti",
    "tra",
    "tu",
    "tua",
    "tue",
    "tuo",
    "tuoi",
    "tutti",
    "tutto",
    "un",
    "una",
    "uno",
    "vi",
    "voi",
    "vostra",
    "vostre",
    "vostri",
    "vostro",
    "è",
];
