
use std::collections::HashMap;

use pipeline::{self, Pipeline};
use inverted_index::InvertedIndex;
use document_store::DocumentStore;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    fields: Vec<String>,
    pipeline: Pipeline,
    #[serde(rename = "ref")] ref_field: String,
    version: &'static str,
    index: HashMap<String, InvertedIndex>,
    document_store: DocumentStore,
}

impl Default for Index {
    fn default() -> Self {
        Index {
            fields: Vec::new(),
            pipeline: Pipeline::default(),
            ref_field: "id".into(),
            version: ::ELASTICLUNR_VERSION,
            index: HashMap::new(),
            document_store: DocumentStore::new(true),
        }
    }
}

impl Index {
    pub fn new(ref_field: &str, fields: &[&str], save_docs: bool) -> Self {
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
            document_store: DocumentStore::new(save_docs),
        }
    }

    pub fn add_field(mut self, field: &str) -> Self {
        self.fields.push(field.into());
        self.index.insert(field.into(), InvertedIndex::new());
        self
    }

    pub fn add_doc(&mut self, doc_ref: &str, doc: HashMap<String, String>) {
        self.document_store.add_doc(doc_ref, &doc);

        let mut token_freq = HashMap::new();
        for (field, value) in &doc {
            if field == &self.ref_field {
                continue;
            }

            let tokens = self.pipeline.run(pipeline::tokenize(value));
            self.document_store
                .add_field_length(doc_ref, field, tokens.len());

            for token in tokens {
                *token_freq.entry(token).or_insert(0u64) += 1;
            }

            for (token, count) in &token_freq {
                let freq = (*count as f64).sqrt();
                self.index.get_mut(field)
                    .expect("Invalid HashMap") // TODO: better API
                    .add_token(doc_ref, token, freq);
            }
        }
    }

    pub fn remove_doc(&mut self, doc_ref: &str) {
        if doc_ref.len() == 0 
            || !self.document_store.is_stored()
            || !self.document_store.has_doc(doc_ref)
        {
            return;
        }

        self.document_store.remove_doc(doc_ref);

        for field in &self.fields {
            let tokens = self.pipeline.run(pipeline::tokenize(field));
            for token in &tokens {
                self.index
                    .get_mut(field.into())
                    .map(|item| item.remove_token(doc_ref, token));
            }
        }
    }

    pub fn update_doc(&mut self, doc_ref: &str, doc: HashMap<String, String>) {
        self.remove_doc(doc_ref);
        self.add_doc(doc_ref, doc);
    }

    pub fn inverse_doc_freq(&self, term: &str, field: &str) -> f64 {
        let df = self.index.get(field).map_or(0, |item| item.get_doc_frequency(term));
        
        1. + f64::ln(self.document_store.len() as f64 / (df + 1) as f64)
    }

    pub fn get_fields(&self) -> &[String] {
        &self.fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inverted_index::TermFrequency;

    #[test]
    fn defining_fields_to_index() {
        let idx = Index::default()
            .add_field("foo");

        assert_eq!(idx.get_fields()[0], "foo");
    }

    #[test]
    fn adding_document_to_index() {
        let mut idx = Index::default();
        let doc = hashmap!{ "id".into() => "1".into(), "body".into() => "this is a test".into() };

        idx = idx.add_field("body");
        idx.add_doc("1", doc.clone());

        assert_eq!(idx.document_store.len(), 1);
        assert_eq!(idx.document_store.get_doc("1"), Some(doc));
    }

    #[test]
    fn adding_document_with_empty_field() {
        let mut idx = Index::default();
        let doc = hashmap!{ "id".into() => "1".into(), "body".into() => "test".into(), "title".into() => "".into() };
        idx = idx.add_field("title")
                 .add_field("body");

        idx.add_doc("1", doc);
        assert_eq!(idx.index["body"].get_doc_frequency("test"), 1);
        assert_eq!(idx.index["body"].get_docs("test").unwrap()["1"].term_freq, 1.);
    }

    // This isn't really used
    #[ignore]
    #[test]
    fn removing_document_from_index() {
        let mut idx = Index::default();
        let doc = hashmap!{ "id".into() => "1".into(), "body".into() => "this is a test".into() };

        idx = idx.add_field("body");
        assert_eq!(idx.document_store.len(), 0);

        idx.add_doc("1", doc);
        assert_eq!(idx.document_store.len(), 1);

        idx.remove_doc("1");
        assert_eq!(idx.document_store.len(), 0);

        assert_eq!(idx.index["body"].has_token("this"), true);
        assert_eq!(idx.index["body"].has_token("test"), true);
        assert_eq!(idx.index["body"].get_node("this"), None);
        assert_eq!(idx.index["body"].get_node("test"), None);
    }

    // This also isn't really used
    #[ignore]
    #[test]
    fn removing_document_from_index_with_more_than_one_document() {
        let mut idx = Index::default();
        let doc1 = hashmap!{ "id".into() => "1".into(), "body".into() => "this is a test".into() };
        let doc2 = hashmap!{ "id".into() => "2".into(), "body".into() => "this is an apple".into() };

        let docs = hashmap!{ "1".into() => TermFrequency{ term_freq: 1. },
                             "2".into() => TermFrequency{ term_freq: 1. }};

        idx = idx.add_field("body");
        assert_eq!(idx.document_store.len(), 0);

        idx.add_doc("1", doc1);
        assert_eq!(idx.document_store.len(), 1);

        idx.add_doc("2", doc2);
        assert_eq!(idx.document_store.len(), 2);

        assert_eq!(idx.index["body"].get_node("this").unwrap().docs, docs);

        idx.remove_doc("1");
        assert_eq!(idx.document_store.len(), 1);

        assert_eq!(idx.index["body"].has_token("this"), true);
        assert_eq!(idx.index["body"].has_token("test"), true);
        assert_eq!(idx.index["body"].has_token("apple"), true);
        assert_eq!(idx.index["body"].get_node("this").unwrap().doc_freq, 1);
        assert_eq!(idx.index["body"].get_node("apple").unwrap().docs, hashmap!{ "2".into() => TermFrequency{ term_freq: 1. } });
        assert_eq!(idx.index["body"].get_node("this").unwrap().docs, hashmap!{ "2".into() => TermFrequency{ term_freq: 1. } });
    }

    #[test]
    fn removing_nonexistant_document_from_index() {
        let mut idx = Index::default();
        let doc = hashmap!{ "id".into() => "1".into(), "body".into() => "this is a test".into()};

        idx = idx.add_field("body");
        assert_eq!(idx.document_store.len(), 0);

        idx.add_doc("1", doc);
        assert_eq!(idx.document_store.len(), 1);

        idx.remove_doc("2");
        assert_eq!(idx.document_store.len(), 1);
    }

    #[test]
    fn updating_document() {
        let mut idx = Index::default();
        let mut doc = hashmap!{ "id".into() => "1".into(), "body".into() => "foo".into()};

        idx = idx.add_field("body");
        idx.add_doc("1", doc.clone());
        assert_eq!(idx.document_store.len(), 1);
        assert!(idx.index["body"].has_token("foo"));

        doc.insert("body".into(), "bar".into());
        idx.update_doc("1", doc);

        assert_eq!(idx.document_store.len(), 1);
        assert!(idx.index["body"].has_token("bar"));
    }
}
