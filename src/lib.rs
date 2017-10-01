// TODO: codegen
#![feature(plugin)]
#![plugin(phf_macros)]

#[macro_use]
extern crate lazy_static;
extern crate phf;
extern crate rust_stemmers;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use std::collections::HashMap;

pub const ELASTICLUNR_VERSION: &str = "0.9.5";

pub mod document_store;
pub mod index;
pub mod inverted_index;
pub mod pipeline;

pub struct IndexBuilder {
    index: index::Index,
    doc_count: usize,
}

impl IndexBuilder {
    pub fn new() -> Self {
        IndexBuilder {
            index: index::Index::new("id", &["title".into(), "body".into()]),
            doc_count: 1,
        }
    }

    pub fn add_document(&mut self, title: &str, body: &str) {
        let mut map = HashMap::new();
        map.insert("id".into(), self.doc_count.to_string());
        map.insert("title".into(), title.into());
        map.insert("body".into(), body.into());
        self.index.add_doc(&self.doc_count.to_string(), map);
        self.doc_count += 1;
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
