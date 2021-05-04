use crate::data::{Localization, Translatable, Translations};
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SexPracticeLocalization {
    pub name: String,
    pub rules: String,
    pub duration: String,
    pub prerequisites: Option<String>,
    pub failed: String
}

impl Localization for SexPracticeLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SexPractice {
    pub id: u32,
    pub src: SourceRefs,
    pub translations: Translations<SexPracticeLocalization>
}

impl Identifiable for SexPractice {
    fn id(&self) -> Id {
        Id::new(Category::SexPractices, self.id)
    }
}

impl Translatable for SexPractice {
    type Localization = SexPracticeLocalization;

    fn translations(&self) -> &Translations<SexPracticeLocalization> {
        &self.translations
    }
}
