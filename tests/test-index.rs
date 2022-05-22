#[macro_use]
extern crate serde_json;

use elasticlunr::*;
use std::fs::File;
use std::path::Path;

const DOCS: &'static [[&'static str; 2]] = &[
    [
        "Chapter 1",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
    ],
    [
        "Chapter 2",
        "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad",
    ],
    [
        "Chapter 3",
        "minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex",
    ],
    [
        "Chapter 4",
        "ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate",
    ],
    [
        "Chapter 5",
        "velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat",
    ],
    ["Chapter 6", "Spatiëring shouldn’t cause a panic."],
];

fn create_index() -> serde_json::Value {
    let mut index = Index::new(&["title", "body"]);

    for (i, doc) in DOCS.iter().enumerate() {
        index.add_doc(&(i + 1).to_string(), doc);
    }
    json!(index)
}

#[cfg(feature = "ja")]
const DOCS_JA: &'static [[&'static str; 2]] = &[
    [
        "第1章",
        "吾輩は猫である。名前はまだ無い。",
    ],
    [
        "第2章",
        "どこで生れたかとんと見当がつかぬ。何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している。",
    ],
    [
        "第3章",
        "吾輩はここで始めて人間というものを見た。しかもあとで聞くとそれは書生という人間中で一番獰悪な種族であったそうだ。この書生というのは時々我々を捕えて煮て食うという話である。しかしその当時は何という考もなかったから別段恐しいとも思わなかった。ただ彼の掌に載せられてスーと持ち上げられた時何だかフワフワした感じがあったばかりである。掌の上で少し落ちついて書生の顔を見たのがいわゆる人間というものの見始であろう。この時妙なものだと思った感じが今でも残っている。",
    ],
    [
        "第4章",
        "第一毛をもって装飾されべきはずの顔がつるつるしてまるで薬缶だ。その後猫にもだいぶ逢ったがこんな片輪には一度も出会わした事がない。のみならず顔の真中があまりに突起している。",
    ],
];

#[cfg(feature = "ja")]
fn create_index_ja() -> serde_json::Value {
    let mut index = Index::with_language(lang::Japanese::new(), &["title", "body"]);

    for (i, doc) in DOCS_JA.iter().enumerate() {
        index.add_doc(&(i + 1).to_string(), doc);
    }
    json!(index)
}

const GENERATE_FIXTURE: bool = false;

fn get_fixture() -> serde_json::Value {
    if GENERATE_FIXTURE {
        let src = create_index();

        let dest = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/searchindex_fixture.json");
        let dest = File::create(&dest).unwrap();
        serde_json::to_writer_pretty(dest, &src).unwrap();

        src
    } else {
        let json = include_str!("searchindex_fixture.json");
        serde_json::from_str(json).expect("Unable to deserialize the fixture")
    }
}

#[cfg(feature = "ja")]
const GENERATE_FIXTURE_JA: bool = false;

#[cfg(feature = "ja")]
fn get_fixture_ja() -> serde_json::Value {
    if GENERATE_FIXTURE_JA {
        let src = create_index_ja();

        let dest = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/searchindex_fixture_ja.json");
        let dest = File::create(&dest).unwrap();
        serde_json::to_writer_pretty(dest, &src).unwrap();

        src
    } else {
        let json = include_str!("searchindex_fixture_ja.json");
        serde_json::from_str(json).expect("Unable to deserialize the fixture of Japanese")
    }
}

#[test]
fn search_index_hasnt_changed_accidentally() {
    let new_index = create_index();
    let fixture_index = get_fixture();

    if new_index != fixture_index {
        panic!("The search index has changed from the fixture");
    }
}

#[cfg(feature = "ja")]
#[test]
fn ja_search_index_hasnt_changed_accidentally() {
    let new_index = create_index_ja();
    let fixture_index = get_fixture_ja();

    if new_index != fixture_index {
        panic!("The search index has changed from the fixture of Japanese");
    }
}
