use pipeline::Pipeline;


pub fn make_pipeline() -> Pipeline {
    Pipeline {
        queue: vec![
            ("trimmer-zh".into(), trimmer),
            ("stopWordFilter-zh".into(), stop_word_filter),
            ("stemmer-zh".into(), stemmer),
        ],
    }
}

pub fn trimmer(token: String) -> Option<String> {
    Some(token)
}

make_stop_word_filter!([
    ""
]);

fn stemmer(token: String) -> Option<String> {
    Some(token)
}
