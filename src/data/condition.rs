use crate::data::{Localization, Translations, Translatable};
use crate::data::src::SourceRefs;
use crate::data::errata::Errata;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct ConditionLocalization {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "levelDescription")]
    pub level_description: Option<String>,
    pub levels: [String; 4],
    pub errata: Option<Errata>
}

impl Localization for ConditionLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct Condition {
    pub id: u32,
    pub src: SourceRefs,
    pub translations: Translations<ConditionLocalization>
}

impl Identifiable for Condition {
    fn id(&self) -> Id {
        Id::new(Category::Conditions,  self.id)
    }
}

impl Translatable<ConditionLocalization> for Condition {
    fn translations(&self) -> &Translations<ConditionLocalization> {
        &self.translations
    }
}
