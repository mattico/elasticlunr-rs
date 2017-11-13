#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate phf;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

/// The version of elasticlunr.js this library was designed for.
pub const ELASTICLUNR_VERSION: &str = "0.9.5";

pub mod document_store;
pub mod index;
pub mod inverted_index;
pub mod pipeline;
mod stemmer;

/// A helper for creating an `Index` for documents with a title and a body field.
pub struct IndexBuilder {
    index: index::Index,
    doc_count: usize,
}

impl IndexBuilder {
    pub fn new() -> Self {
        IndexBuilder {
            index: index::Index::new("id", &["title", "body"], true),
            doc_count: 1,
        }
    }

    pub fn add_document(&mut self, title: &str, body: &str) {
        let doc_count = self.doc_count.to_string();
        let map = hashmap!{
            "id".into() => doc_count.clone(),
            "title".into() => title.into(),
            "body".into() => body.into(),
        };
        self.index.add_doc(&doc_count, map);
        self.doc_count += 1;
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.index).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.index).unwrap()
    }
}
