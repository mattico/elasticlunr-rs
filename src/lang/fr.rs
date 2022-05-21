use crate::pipeline::Pipeline;

pub fn make_pipeline() -> Pipeline {
    Pipeline {
        queue: vec![
            ("trimmer-fr".into(), trimmer),
            ("stopWordFilter-fr".into(), stop_word_filter),
            ("stemmer-fr".into(), stemmer),
        ],
    }
}

make_trimmer!(
    "A-Za-z\\xAA\\xBA\\xC0-\\xD6\\xD8-\\xF6\\xF8-\\u02B8\\u02E0-\\u02E4\\u1D00-\\u1D25\
     \\u1D2C-\\u1D5C\\u1D62-\\u1D65\\u1D6B-\\u1D77\\u1D79-\\u1DBE\\u1E00-\\u1EFF\\u2071\\u207F\
     \\u2090-\\u209C\\u212A\\u212B\\u2132\\u214E\\u2160-\\u2188\\u2C60-\\u2C7F\\uA722-\\uA787\
     \\uA78B-\\uA7AD\\uA7B0-\\uA7B7\\uA7F7-\\uA7FF\\uAB30-\\uAB5A\\uAB5C-\\uAB64\\uFB00-\\uFB06\
     \\uFF21-\\uFF3A\\uFF41-\\uFF5A"
);

make_stop_word_filter!([
    "", "ai", "aie", "aient", "aies", "ait", "as", "au", "aura", "aurai", "auraient", "aurais",
    "aurait", "auras", "aurez", "auriez", "aurions", "aurons", "auront", "aux", "avaient", "avais",
    "avait", "avec", "avez", "aviez", "avions", "avons", "ayant", "ayez", "ayons", "c", "ce",
    "ceci", "celà", "ces", "cet", "cette", "d", "dans", "de", "des", "du", "elle", "en", "es",
    "est", "et", "eu", "eue", "eues", "eurent", "eus", "eusse", "eussent", "eusses", "eussiez",
    "eussions", "eut", "eux", "eûmes", "eût", "eûtes", "furent", "fus", "fusse", "fussent",
    "fusses", "fussiez", "fussions", "fut", "fûmes", "fût", "fûtes", "ici", "il", "ils", "j",
    "je", "l", "la", "le", "les", "leur", "leurs", "lui", "m", "ma", "mais", "me", "mes", "moi",
    "mon", "même", "n", "ne", "nos", "notre", "nous", "on", "ont", "ou", "par", "pas", "pour",
    "qu", "que", "quel", "quelle", "quelles", "quels", "qui", "s", "sa", "sans", "se", "sera",
    "serai", "seraient", "serais", "serait", "seras", "serez", "seriez", "serions", "serons",
    "seront", "ses", "soi", "soient", "sois", "soit", "sommes", "son", "sont", "soyez", "soyons",
    "suis", "sur", "t", "ta", "te", "tes", "toi", "ton", "tu", "un", "une", "vos", "votre", "vous",
    "y", "à", "étaient", "étais", "était", "étant", "étiez", "étions", "été", "étée",
    "étées", "étés", "êtes",
]);

make_stemmer!(Algorithm::French);
