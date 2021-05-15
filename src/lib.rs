use serde::{Deserialize, Serialize};

pub mod character;
pub mod compatibility;
pub mod data;
pub mod error;
pub mod id;
pub mod util;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum Sex {
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "m")]
    Male
}
