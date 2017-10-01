
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DocumentStore {
    save: bool,
    docs: HashMap<String, HashMap<String, String>>,
    doc_info: HashMap<String, HashMap<String, usize>>,
    length: usize,
}

impl DocumentStore {
    pub fn new() -> Self {
        DocumentStore {
            save: true,
            docs: HashMap::new(),
            doc_info: HashMap::new(),
            length: 0,
        }
    }

    pub fn is_doc_stored(&self) -> bool {
        self.save
    }

    pub fn has_doc(&self, doc_ref: &str) -> bool {
        self.docs.contains_key(doc_ref)
    }

    pub fn add_doc(&mut self, doc_ref: &str, doc: &HashMap<String, String>) {
        if !self.has_doc(doc_ref) {
            self.length += 1;
        }
        
        self.docs.insert(doc_ref.into(), if self.save { doc.clone() } else { HashMap::new() });
    }

    pub fn get_doc(&self, doc_ref: &str) -> Option<HashMap<String, String>> {
        self.docs.get(doc_ref.into()).cloned()
    }

    pub fn add_field_length(&mut self, doc_ref: &str, field: &str, length: usize) {
        if !self.has_doc(doc_ref) { return; }
        self.doc_info.entry(doc_ref.into())
            .or_insert(HashMap::new())
            .insert(field.into(), length);
    }
}
