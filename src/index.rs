

#[derive(Serialize, Debug)]
pub struct Index {
    fields: &'static [&'static str],
    pipeline: ::pipeline::Pipeline,
    #[serde(rename = "ref")]
    reference: &'static str,
    version: &'static str,
    index: ::inverted_index::InvertedIndex,
    document_store: ::document_store::DocumentStore,
}