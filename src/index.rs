
use std::collections::HashMap;

use pipeline::{self, Pipeline};
use inverted_index::InvertedIndex;
use document_store::DocumentStore;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    fields: Vec<String>,
    pipeline: Pipeline,
    #[serde(rename = "ref")]
    ref_field: String,
    version: &'static str,
    index: HashMap<String, InvertedIndex>,
    document_store: DocumentStore,
}

impl Index {
    pub fn new(ref_field: &str, fields: &[&str]) -> Self {
        let mut indices = HashMap::new();
        for field in fields {
            indices.insert((*field).into(), InvertedIndex::new());
        }

        Index {
            fields: fields.iter().map(ToString::to_string).collect(),
            pipeline: Pipeline::default(),
            ref_field: ref_field.into(),
            version: ::ELASTICLUNR_VERSION,
            index: indices,
            document_store: DocumentStore::new(true),
        }
    }

    pub fn add_doc(&mut self, doc_ref: &str, doc: HashMap<String, String>) {
        self.document_store.add_doc(doc_ref, &doc);

        let mut token_freq = HashMap::new();
        for (field, value) in &doc {
            if field == &self.ref_field {
                continue;
            }

            let tokens = self.pipeline.run(pipeline::tokenize(value));
            self.document_store.add_field_length(
                doc_ref,
                field,
                tokens.len(),
            );

            for token in tokens {
                *token_freq.entry(token).or_insert(0u64) += 1;
            }

            for (token, count) in &token_freq {
                let freq = (*count as f32).sqrt();
                self.index.get_mut(field)
                    .expect("Invalid HashMap") // TODO: better API
                    .add_token(doc_ref, token, freq);
            }
        }
    }
}
