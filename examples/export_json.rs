extern crate elasticlunr;

use std::fs;
use std::io::Write;
use elasticlunr::IndexBuilder;

fn main() {
    let mut index_builder = IndexBuilder::new();
    index_builder.add_document("This Week in Rust 207",
        "Hello and welcome to another issue of This Week in Rust! Rust is a systems language pursuing the trifecta: safety, concurrency, and speed. This is a weekly summary of its progress and community. Want something mentioned? Tweet us at @ThisWeekInRust or send us a pull request. Want to get involved? We love contributions. This Week in Rust is openly developed on GitHub. If you find any errors in this week's issue, please submit a PR.");
    index_builder.add_document("This Week in Rust 206",
        "Hello and welcome to another issue of This Week in Rust! Rust is a systems language pursuing the trifecta: safety, concurrency, and speed. This is a weekly summary of its progress and community. Want something mentioned? Tweet us at @ThisWeekInRust or send us a pull request. Want to get involved? We love contributions. This Week in Rust is openly developed on GitHub. If you find any errors in this week's issue, please submit a PR.");
    let mut file = fs::File::create("examples/out.json").unwrap();
    file.write_all(index_builder.to_json_pretty().as_bytes());
}
