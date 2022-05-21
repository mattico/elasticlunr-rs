use crate::pipeline::Pipeline;

pub fn make_pipeline() -> Pipeline {
    Pipeline {
        queue: vec![
            ("trimmer-no".into(), trimmer),
            ("stopWordFilter-no".into(), stop_word_filter),
            ("stemmer-no".into(), stemmer),
        ],
    }
}

make_trimmer!(
    "A-Za-z\\xAA\\xBA\\xC0-\\xD6\\xD8-\\xF6\\xF8-\\u02B8\\u02E0-\\u02E4\\u1D00-\\u1D25\\u1D2C-\
    \\u1D5C\\u1D62-\\u1D65\\u1D6B-\\u1D77\\u1D79-\\u1DBE\\u1E00-\\u1EFF\\u2071\\u207F\\u2090-\
    \\u209C\\u212A\\u212B\\u2132\\u214E\\u2160-\\u2188\\u2C60-\\u2C7F\\uA722-\\uA787\\uA78B-\
    \\uA7AD\\uA7B0-\\uA7B7\\uA7F7-\\uA7FF\\uAB30-\\uAB5A\\uAB5C-\\uAB64\\uFB00-\\uFB06\\uFF21-\
    \\uFF3A\\uFF41-\\uFF5A"
);

make_stop_word_filter!([
    "", "alle", "at", "av", "bare", "begge", "ble", "blei", "bli", "blir", "blitt", "både", "båe",
    "da", "de", "deg", "dei", "deim", "deira", "deires", "dem", "den", "denne", "der", "dere",
    "deres", "det", "dette", "di", "din", "disse", "ditt", "du", "dykk", "dykkar", "då", "eg",
    "ein", "eit", "eitt", "eller", "elles", "en", "enn", "er", "et", "ett", "etter", "for",
    "fordi", "fra", "før", "ha", "hadde", "han", "hans", "har", "hennar", "henne", "hennes", "her",
    "hjå", "ho", "hoe", "honom", "hoss", "hossen", "hun", "hva", "hvem", "hver", "hvilke",
    "hvilken", "hvis", "hvor", "hvordan", "hvorfor", "i", "ikke", "ikkje", "ikkje", "ingen",
    "ingi", "inkje", "inn", "inni", "ja", "jeg", "kan", "kom", "korleis", "korso", "kun", "kunne",
    "kva", "kvar", "kvarhelst", "kven", "kvi", "kvifor", "man", "mange", "me", "med", "medan",
    "meg", "meget", "mellom", "men", "mi", "min", "mine", "mitt", "mot", "mykje", "ned", "no",
    "noe", "noen", "noka", "noko", "nokon", "nokor", "nokre", "nå", "når", "og", "også", "om",
    "opp", "oss", "over", "på", "samme", "seg", "selv", "si", "si", "sia", "sidan", "siden", "sin",
    "sine", "sitt", "sjøl", "skal", "skulle", "slik", "so", "som", "som", "somme", "somt", "så",
    "sånn", "til", "um", "upp", "ut", "uten", "var", "vart", "varte", "ved", "vere", "verte", "vi",
    "vil", "ville", "vore", "vors", "vort", "vår", "være", "være", "vært", "å"
]);

make_stemmer!(Algorithm::Norwegian);
