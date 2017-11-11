
use std::collections::HashMap;
use serde::ser::{Serialize, SerializeMap, Serializer};

#[derive(Debug, Copy, Clone, Serialize)]
pub struct TermFrequency {
    #[serde(rename = "tf")] term_freq: f32,
}

#[derive(Debug, Clone)]
pub struct IndexItem {
    docs: HashMap<String, TermFrequency>,
    doc_freq: i64,
    children: HashMap<String, IndexItem>,
}

impl IndexItem {
    pub fn new() -> Self {
        IndexItem {
            docs: HashMap::new(),
            doc_freq: 0,
            children: HashMap::new(),
        }
    }

    fn add_update_children(&mut self, token: &str, term_freq: f32) {
        if let Some((char_idx, _)) = token.char_indices().next() {
            let (char, rest) = token.split_at(char_idx);
            self.children
                .entry(char.into())
                .or_insert(IndexItem::new())
                .add_update_children(rest.into(), term_freq);
        }
    }

    pub fn add_token(&mut self, doc_ref: &str, token: &str, term_freq: f32) {
        self.add_update_children(token, term_freq);

        if !self.docs.contains_key(doc_ref.into()) {
            self.doc_freq += 1;
        }
        self.docs
            .insert(doc_ref.into(), TermFrequency { term_freq });
    }

    pub fn has_token(&self, token: &str) -> bool {
        let mut root = self;
        for char in token.chars() {
            if let Some(item) = root.children.get(&char.to_string()) {
                root = item;
            } else {
                return false;
            }
        }

        true
    }

    pub fn get_node(&self, token: &str) -> Option<&IndexItem> {
        let mut root = self;
        for char in token.chars() {
            if let Some(item) = root.children.get(&char.to_string()) {
                root = item;
            } else {
                return None;
            }
        }

        Some(self)
    }

    pub fn remove_token(&mut self, doc_ref: &str, token: &str) {
        if let Some((char_idx, _)) = token.char_indices().next() {
            let (char, rest) = token.split_at(char_idx).into();
            if let Some(item) = self.children.get_mut(char) {
                item.remove_token(doc_ref, rest);
            } else {
                return;
            }
        }

        self.docs.remove(doc_ref);
        self.doc_freq -= 1;
    }

    pub fn expand_token(&self, token: String, expanded: &mut Vec<String>) {
        if token.len() == 0 { return; }

        if let Some(root) = self.get_node(&token) {
            if root.doc_freq > 0 {
                expanded.push(token.clone());
            }
            let mut token = token;
            for (key, val) in &root.children {
                token.push_str(&key);
                val.expand_token(token.clone(), expanded);
            }
        }
    }
}

// Manually implement serialize so `children` are inline
impl Serialize for IndexItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(2 + self.children.len()))?;
        state.serialize_entry("df", &self.doc_freq)?;
        state.serialize_entry("docs", &self.docs)?;

        for (key, value) in &self.children {
            state.serialize_entry(key, &value)?;
        }

        state.end()
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

    pub fn add_token(&mut self, doc_ref: &str, token: &str, term_freq: f32) {
        self.root.add_token(doc_ref, token, term_freq)
    }

    pub fn has_token(&self, token: &str) -> bool {
        self.root.get_node(token).map_or(false, |_| true)
    }

    pub fn remove_token(&mut self, doc_ref: &str, token: &str) {
        self.root.remove_token(doc_ref, token)
    }

    pub fn get_docs(&self, token: &str) -> Option<&HashMap<String, TermFrequency>> {
        self.root.get_node(token).and_then(|node| Some(&node.docs))
    }

    pub fn get_term_frequency(&self, doc_ref: &str, token: &str) -> f32 {
        self.root
            .get_node(token)
            .and_then(|node| node.docs.get(doc_ref.into()))
            .map_or(0., |docs| docs.term_freq)
    }

    pub fn get_doc_frequency(&self, token: &str) -> i64 {
        self.root.get_node(token).map_or(0, |node| node.doc_freq)
    }

    pub fn expand_token(&self, token: &str) -> Vec<String> {
        let mut buf = vec![];
        self.root.expand_token(token.into(), &mut buf);
        buf
    }
}
