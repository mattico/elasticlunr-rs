#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate phf;
extern crate rust_stemmers;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

/// The version of elasticlunr.js this library was designed for.
pub const ELASTICLUNR_VERSION: &str = "0.9.5";

pub mod document_store;
pub mod index;
pub mod inverted_index;
pub mod pipeline;

/// A helper for creating an `Index` with a title and a body field.
pub struct IndexBuilder {
    index: index::Index,
    doc_count: usize,
}

impl IndexBuilder {
    pub fn new() -> Self {
        IndexBuilder {
            index: index::Index::new("id", &["title".into(), "body".into()], true),
            doc_count: 1,
        }
    }

    pub fn add_document(&mut self, title: &str, body: &str) {
        let map = hashmap!{
            "id".into() => self.doc_count.to_string(),
            "title".into() => title.into(),
            "body".into() => body.into(),
        };
        self.index.add_doc(&self.doc_count.to_string(), map);
        self.doc_count += 1;
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.index).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.index).unwrap()
    }
}
