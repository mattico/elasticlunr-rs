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

pub const ELASTICLUNR_VERSION: &str = "0.9.5";

pub mod document_store;
pub mod index;
pub mod inverted_index;
pub mod pipeline;

pub struct IndexBuilder {
    index: index::Index,
}

impl IndexBuilder {
    // pub fn new() -> Self {
    //     IndexBuilder {
    //         index: index::Index {
    //             fields: &["title", "body"],
    //             pipeline: &["trimmer", "stopWordFilter", "stemmer"],
    //             reference: "id",
    //             version: ELASTICLUNR_VERSION,
    //             index: BTreeMap::new(),
    //         },
    //     }
    // }

    // pub fn add_document<T: Into<String>>(&mut self, title: T, body: T) {
    //     self.index.index.insert(title.into(), body.into());
    // }

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
