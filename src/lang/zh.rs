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
    let ret: String = token.
        trim_matches(|c: char| !is_valid_char(c)  )
        .into();

    if ret.eq("") {
        return None;
    }

    Some(ret)
}

make_stop_word_filter!([
    "的", "了"
]);

fn stemmer(token: String) -> Option<String> {
    Some(token)
}

fn is_valid_char(c: char) -> bool {
    let min_max_list = [
        [19668, 40869], // min and max Chinese char
        ['a' as u32, 'z' as u32],
        ['A' as u32, 'Z' as u32]
    ];

    let c = c as u32;
    for min_max in min_max_list.iter() {
        if c >= min_max[0] && c <= min_max[1] {
            return true;
        }
    }

    false
}