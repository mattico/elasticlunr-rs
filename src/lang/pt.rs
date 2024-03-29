use super::{
    common::{RustStemmer, StopWordFilter, RegexTrimmer},
    Language,
};
use crate::pipeline::Pipeline;
use rust_stemmers::Algorithm;

#[derive(Clone)]
pub struct Portuguese {}

impl Portuguese {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for Portuguese {
    fn name(&self) -> String {
        "Portuguese".into()
    }
    fn code(&self) -> String {
        "pt".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(RegexTrimmer::new("trimmer-pt", r"\p{Latin}")),
                Box::new(StopWordFilter::new("stopWordFilter-pt", STOP_WORDS)),
                Box::new(RustStemmer::new("stemmer-pt", Algorithm::Portuguese)),
            ],
        }
    }
}

const STOP_WORDS: &[&str] = &[
    "",
    "a",
    "ao",
    "aos",
    "aquela",
    "aquelas",
    "aquele",
    "aqueles",
    "aquilo",
    "as",
    "até",
    "com",
    "como",
    "da",
    "das",
    "de",
    "dela",
    "delas",
    "dele",
    "deles",
    "depois",
    "do",
    "dos",
    "e",
    "ela",
    "elas",
    "ele",
    "eles",
    "em",
    "entre",
    "era",
    "eram",
    "essa",
    "essas",
    "esse",
    "esses",
    "esta",
    "estamos",
    "estas",
    "estava",
    "estavam",
    "este",
    "esteja",
    "estejam",
    "estejamos",
    "estes",
    "esteve",
    "estive",
    "estivemos",
    "estiver",
    "estivera",
    "estiveram",
    "estiverem",
    "estivermos",
    "estivesse",
    "estivessem",
    "estivéramos",
    "estivéssemos",
    "estou",
    "está",
    "estávamos",
    "estão",
    "eu",
    "foi",
    "fomos",
    "for",
    "fora",
    "foram",
    "forem",
    "formos",
    "fosse",
    "fossem",
    "fui",
    "fôramos",
    "fôssemos",
    "haja",
    "hajam",
    "hajamos",
    "havemos",
    "hei",
    "houve",
    "houvemos",
    "houver",
    "houvera",
    "houveram",
    "houverei",
    "houverem",
    "houveremos",
    "houveria",
    "houveriam",
    "houvermos",
    "houverá",
    "houverão",
    "houveríamos",
    "houvesse",
    "houvessem",
    "houvéramos",
    "houvéssemos",
    "há",
    "hão",
    "isso",
    "isto",
    "já",
    "lhe",
    "lhes",
    "mais",
    "mas",
    "me",
    "mesmo",
    "meu",
    "meus",
    "minha",
    "minhas",
    "muito",
    "na",
    "nas",
    "nem",
    "no",
    "nos",
    "nossa",
    "nossas",
    "nosso",
    "nossos",
    "num",
    "numa",
    "não",
    "nós",
    "o",
    "os",
    "ou",
    "para",
    "pela",
    "pelas",
    "pelo",
    "pelos",
    "por",
    "qual",
    "quando",
    "que",
    "quem",
    "se",
    "seja",
    "sejam",
    "sejamos",
    "sem",
    "serei",
    "seremos",
    "seria",
    "seriam",
    "será",
    "serão",
    "seríamos",
    "seu",
    "seus",
    "somos",
    "sou",
    "sua",
    "suas",
    "são",
    "só",
    "também",
    "te",
    "tem",
    "temos",
    "tenha",
    "tenham",
    "tenhamos",
    "tenho",
    "terei",
    "teremos",
    "teria",
    "teriam",
    "terá",
    "terão",
    "teríamos",
    "teu",
    "teus",
    "teve",
    "tinha",
    "tinham",
    "tive",
    "tivemos",
    "tiver",
    "tivera",
    "tiveram",
    "tiverem",
    "tivermos",
    "tivesse",
    "tivessem",
    "tivéramos",
    "tivéssemos",
    "tu",
    "tua",
    "tuas",
    "tém",
    "tínhamos",
    "um",
    "uma",
    "você",
    "vocês",
    "vos",
    "à",
    "às",
    "éramos",
];
