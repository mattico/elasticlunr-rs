extern crate elasticlunr;

use elasticlunr::Index;
use elasticlunr::Language;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut index = Index::with_language(Language::Chinese, &["title", "body"], );
    index.add_doc(
        "1",
        &[
            "中华人民共和国",
            "杭州余杭区人民欢迎你"
        ],
    );

//    index.add_doc(
//        "1",
//        &[
//            "This Week in Rust 207",
//            "Hello and welcome to another issue of This Week in Rust!",
//        ],
//    );
//    index.add_doc(
//        "2",
//        &[
//            "This Week in Rust 206",
//            "Hello and welcome to another issue of This Week in Rust!",
//        ],
//    );
    let mut file = File::create("examples/out.json").unwrap();
    file.write_all(index.to_json_pretty().as_bytes()).unwrap();
}
