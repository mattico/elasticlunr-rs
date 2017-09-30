
use std::collections::HashMap;

// TODO: manually implement serialize so children are inline
#[derive(Serialize, Debug)]
pub struct IndexItem {
    docs: HashMap<String, String>,
    df: usize,
    children: HashMap<String, IndexItem>,
}

#[derive(Serialize, Debug)]
pub struct InvertedIndex {
    root: IndexItem,
}

