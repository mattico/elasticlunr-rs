#[macro_use]
extern crate serde_json;
extern crate elasticlunr;

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
];

fn create_index() -> serde_json::Value {
    let mut index = Index::new(&["title", "body"]);

    let mut i = 1;
    for doc in DOCS.iter() {
        index.add_doc(&format!("{}", i), doc);
        i += 1;
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

#[test]
fn search_index_hasnt_changed_accidentally() {
    let new_index = create_index();
    let fixture_index = get_fixture();

    if new_index != fixture_index {
        panic!("The search index has changed from the fixture");
    }
}
