
use std::collections::HashMap;

// TODO: manually implement serialize so children are inline
#[derive(Serialize, Debug, Clone)]
pub struct IndexItem {
    docs: HashMap<String, String>,
    df: usize,
    children: HashMap<String, IndexItem>,
}

pub struct TokenInfo {}

impl IndexItem {
    pub fn new() -> Self {
        IndexItem {
            docs: HashMap::new(),
            df: 0,
            children: HashMap::new(),
        }
    }

    pub fn add_token(&mut self, token: &str) 
    {
        if let Some((idx, char)) = token.char_indices().next() {
            let item = self.children.entry(char.to_string()).or_insert(IndexItem::new());
            item.add_token(&token[idx..]);
        }
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

    pub fn add_token(&mut self, token: &str, token_info: TokenInfo) 
    {
        self.root.add_token(token);
        // TODO: handle docs, tokeninfo
    }
}

