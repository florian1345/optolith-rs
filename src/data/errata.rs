use crate::data::{Localization, Translations};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Erratum {
    pub date: String,
    pub description: String
}

pub type Errata = Vec<Erratum>;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ErrataLocalization {
    pub name: String,
    pub errata: Option<Errata>
}

impl Localization for ErrataLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

pub type ErrataTranslations = Translations<ErrataLocalization>;
