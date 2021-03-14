use serde::{Deserialize, Serialize};

use crate::data::{Localization, Translatable, Translations};
use crate::id::{Category, Id, Identifiable};

#[derive(Deserialize, Serialize)]
pub struct AttributeLocalization {
    pub name: String,
    #[serde(rename = "nameAbbr")]
    pub name_abbr: String
}

impl Localization for AttributeLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct Attribute {
    pub id: u32,
    pub translations: Translations<AttributeLocalization>
}

impl Identifiable for Attribute {
    fn id(&self) -> Id {
        Id::new(Category::Attributes, self.id)
    }
}

impl Translatable for Attribute {
    type Localization = AttributeLocalization;

    fn translations(&self) -> &Translations<AttributeLocalization> {
        &self.translations
    }
}
