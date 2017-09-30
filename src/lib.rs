extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;

pub const ELASTICLUNR_VERSION: &str = "0.9.5";

pub struct IndexBuilder {
    index: Index,
}

#[derive(Serialize)]
struct Index {
    fields: &'static [&'static str],
    pipeline: &'static [&'static str],
    #[serde(rename = "ref")]
    reference: &'static str,
    version: &'static str,
    index: BTreeMap<String, String>,
    //document_store
}

impl IndexBuilder {
    pub fn new() -> Self {
        IndexBuilder {
            index: Index {
                fields: &["title", "body"],
                pipeline: &["trimmer", "stopWordFilter", "stemmer"],
                reference: "id",
                version: ELASTICLUNR_VERSION,
                index: BTreeMap::new(),
            },
        }
    }

    pub fn add_document<T: Into<String>>(&mut self, title: T, body: T) {
        self.index.index.insert(title.into(), body.into());
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
