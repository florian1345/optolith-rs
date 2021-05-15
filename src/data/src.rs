use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Occurrence {
    #[serde(rename = "firstPage")]
    pub first_page: u32,
    #[serde(rename = "lastPage")]
    pub last_page: Option<u32>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum Occurrences {
    One(Occurrence),
    Many(Vec<Occurrence>)
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRef {
    pub id: u32,

    /// The book's universal printing identifier since which the entry is
    /// present.
    pub since: Option<u32>,

    /// The book's universal printing identifier since which the entry has been
    /// removed.
    pub deprecated: Option<u32>,

    /// Maps the language ID to the occurrences in the books of that language.
    pub occurrences: HashMap<String, Occurrences>
}

pub type SourceRefs = Vec<SourceRef>;
