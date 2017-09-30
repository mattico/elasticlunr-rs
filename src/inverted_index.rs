
use std::collections::HashMap;

// TODO: manually implement serialize so children are inline
// and docs values are objects
#[derive(Serialize, Debug, Clone)]
pub struct IndexItem {
    docs: HashMap<String, f32>,
    df: usize,
    children: HashMap<String, IndexItem>,
}

impl IndexItem {
    pub fn new() -> Self {
        IndexItem {
            docs: HashMap::new(),
            df: 0,
            children: HashMap::new(),
        }
    }

    pub fn add_token(&mut self, token: &str, doc_ref: &str, freq: f32) 
    {
        if let Some((idx, char)) = token.char_indices().next() {
            let item = self.children.entry(char.to_string()).or_insert(IndexItem::new());
            item.add_token(&token[idx..], doc_ref, freq);
        }

        if self.docs.contains_key(doc_ref) { self.df += 1; }
        self.docs.insert(doc_ref.into(), freq);
    }
}

#[derive(Serialize, Debug)]
pub struct InvertedIndex {
    root: IndexItem,
}

impl InvertedIndex {
    pub fn new() -> Self {
        InvertedIndex {
            root: IndexItem::new(),
        }
    }

    pub fn add_token(&mut self, token: &str, doc_ref: &str, freq: f32) 
    {
        self.root.add_token(token, doc_ref, freq);
    }
}

