
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct Document {
    pub id: usize,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Debug)]
pub struct DocumentInfo {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocumentStore {
    save: bool,
    docs: HashMap<String, Document>,
    doc_info: HashMap<String, DocumentInfo>,
    length: usize,
}

impl DocumentStore {
    pub fn new() -> Self {
        DocumentStore {
            save: false,
            docs: HashMap::new(),
            doc_info: HashMap::new(),
            length: 0,
        }
    }

    pub fn is_doc_stored(&self) -> bool {
        self.save
    }

    pub fn add_doc(&mut self, reference: String, title: String, body: String) {
        // TODO: only insert if self.save
        self.docs.insert(reference, Document {
            id: self.length + 1,
            title,
            body,
        });
        // TODO: docInfo
        // TODO: only increment length when doc isn't already existing
        self.length += 1;
    }
}
