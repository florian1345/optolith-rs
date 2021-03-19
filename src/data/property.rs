use crate::data::Translatable;
use crate::data::simple::{SimpleLocalization, SimpleTranslations};
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Property {
    pub id: u32,
    pub check: Option<[u32; 3]>, // TODO resolve optionality
    pub translations: SimpleTranslations
}

impl Identifiable for Property {
    fn id(&self) -> Id {
        Id::new(Category::Properties, self.id)
    }
}

impl Translatable for Property {
    type Localization = SimpleLocalization;

    fn translations(&self) -> &SimpleTranslations {
        &self.translations
    }
}
