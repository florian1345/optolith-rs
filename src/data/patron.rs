use crate::data::Translatable;
use crate::data::simple::{SimpleLocalization, SimpleTranslations};
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Patron {
    pub id: u32,

    /// The category of the patron.
    pub category: u32,

    /// The patron-specific skills.
    pub skills: [u32; 3],

    /// If defined, the patron is limited to the listed cultures.
    #[serde(rename = "limitedToCultures")]
    pub limited_to_cultures: Option<Vec<u32>>,

    /// If `true`, the patron is limited to every culture *except* the listed
    /// cultures in `limitedToCultures`. Does not have an effect if
    /// `limitedToCultures` is not defined.
    #[serde(rename = "isLimitedToCulturesReverse")]
    pub is_limited_to_cultures_reverse: Option<bool>,
    pub translations: SimpleTranslations
}

impl Identifiable for Patron {
    fn id(&self) -> Id {
        Id::new(Category::Patrons, self.id)
    }
}

impl Translatable for Patron {
    type Localization = SimpleLocalization;

    fn translations(&self) -> &SimpleTranslations {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
pub struct PatronCategory {
    pub id: u32,

    /// The list of cultures where patrons from this category can be the
    /// primary patron.
    #[serde(rename = "primaryPatronCultures")]
    pub primary_patron_cultures: Vec<u32>,
    pub translations: SimpleTranslations
}

impl Identifiable for PatronCategory {
    fn id(&self) -> Id {
        Id::new(Category::PatronCategories, self.id)
    }
}

impl Translatable for PatronCategory {
    type Localization = SimpleLocalization;

    fn translations(&self) -> &SimpleTranslations {
        &self.translations
    }
}
