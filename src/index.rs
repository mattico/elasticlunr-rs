
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct Index {
    fields: &'static [&'static str],
    pipeline: &'static [&'static str],
    #[serde(rename = "ref")]
    reference: &'static str,
    version: &'static str,
    index: BTreeMap<String, String>,
    //document_store
}