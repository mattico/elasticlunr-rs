
use std::collections::HashMap;

use ::pipeline::{self, Pipeline};
use ::inverted_index::InvertedIndex;
use ::document_store::DocumentStore;

#[derive(Serialize, Debug)]
pub struct Index {
    fields: &'static [&'static str],
    pipeline: Pipeline,
    #[serde(rename = "ref")]
    reference: &'static str,
    version: &'static str,
    index: InvertedIndex,
    document_store: DocumentStore,
}

impl Index {
    pub fn new() -> Self {
        Index {
            fields: &["title", "body"],
            pipeline: Pipeline::default(),
            reference: "id",
            version: ::ELASTICLUNR_VERSION,
            index: InvertedIndex::new(),
            document_store: DocumentStore::new(),
        }
    }

    pub fn add_doc(&mut self, id: &str, title: &str, body: &str) {
        let title_tok = self.pipeline.run(pipeline::tokenize(title));
        let body_tok = self.pipeline.run(pipeline::tokenize(body));

        self.add_tokens(id, title_tok);
        self.add_tokens(id, body_tok);
    }

    fn add_tokens(&mut self, id: &str, tokens: Vec<String>) {
        //documentstore.addFieldLength

        let mut token_freq = HashMap::new();

        for token in tokens {
            token_freq.entry(token).or_insert(0u64);
        }

        for (token, count) in token_freq {
            self.index.add_token(&token, id, (count as f32).sqrt() as i64);
        }
    }
}