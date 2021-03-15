use crate::data::{Localization, Translations};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Erratum {
    pub date: String,
    pub description: String
}

pub type Errata = Vec<Erratum>;

#[derive(Deserialize, Serialize)]
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
