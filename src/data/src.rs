use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Occurrence {
    #[serde(rename = "firstPage")]
    pub first_page: u32,
    #[serde(rename = "lastPage")]
    pub last_page: Option<u32>
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Occurrences {
    One(Occurrence),
    Many(Vec<Occurrence>)
}

#[derive(Deserialize, Serialize)]
pub struct SourceRef {
    pub id: u32,

    /// Maps the language ID to the occurrences in the books of that language.
    pub occurrences: HashMap<String, Occurrences>
}

pub type SourceRefs = Vec<SourceRef>;
