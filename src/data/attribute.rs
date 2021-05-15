use serde::{Deserialize, Serialize};

use crate::data::{Localization, TranslationsTranslatable, Translations};
use crate::id::{Category, Id, Identifiable};

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttributeLocalization {
    pub name: String,
    #[serde(rename = "nameAbbr")]
    pub name_abbr: String,
    pub description: String
}

impl Localization for AttributeLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Attribute {
    pub id: u32,
    pub translations: Translations<AttributeLocalization>
}

impl Identifiable for Attribute {
    fn id(&self) -> Id {
        Id::new(Category::Attributes, self.id)
    }
}

impl TranslationsTranslatable for Attribute {
    type Localization = AttributeLocalization;

    fn translations(&self) -> &Translations<AttributeLocalization> {
        &self.translations
    }
}
