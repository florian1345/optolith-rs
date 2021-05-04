use crate::data::Localization;
use crate::data::errata::Errata;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LabelText {

    /// A label that is displayed and placed before the actual text.
    pub label: String,

    /// The effect text.
    pub text: String
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EffectsLocalization {
    pub name: String,

    /// The effects of the influence. They should be sorted like they are in
    /// the book.
    pub effects: Vec<LabelText>,
    pub errata: Option<Errata>
}

impl Localization for EffectsLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}
