use crate::data::{Localization, Translatable, Translations};
use crate::data::errata::{Errata, ErrataLocalization, ErrataTranslations};
use crate::data::simple::SimpleTranslations;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LanguageSpecialization {
    pub id: u32,
    pub translations: SimpleTranslations
}

#[derive(Deserialize, Serialize)]
pub struct LanguageLocalization {
    pub name: String,

    /// If specialization cannot be specified by a list, e.g. specializations
    /// for different tribes, insert the description for the input field here.
    #[serde(rename = "specializationInput")]
    pub specialization_input: Option<String>,
    pub errata: Option<Errata>
}

impl Localization for LanguageLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct Language {
    pub id: u32,

    /// The max level. Only specific, if lower than default of 3.
    #[serde(rename = "maxLevel")]
    pub max_level: Option<u32>,

    /// A list of language-specific specializations.
    pub specializations: Option<Vec<LanguageSpecialization>>,

    /// The continent ID.
    pub continent: u32,

    /// Is the language considered virtually extinct in it's respective
    /// continent?
    #[serde(rename = "isExtinct")]
    pub is_extinct: bool,
    pub src: SourceRefs,
    pub translations: Translations<LanguageLocalization>
}

impl Translatable for Language {
    type Localization = LanguageLocalization;

    fn translations(&self) -> &Translations<LanguageLocalization> {
        &self.translations
    }
}

impl Identifiable for Language {
    fn id(&self) -> Id {
        Id::new(Category::Languages, self.id)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Script {
    pub id: u32,

    /// AP value of the script.
    #[serde(rename = "apValue")]
    pub ap_value: u32,

    /// A list of languages working with the script.
    pub languages: Vec<u32>,

    /// The continent ID.
    pub continent: u32,

    /// Is the script considered virtually extinct in it's respective
    /// continent?
    #[serde(rename = "isExtinct")]
    pub is_extinct: bool,
    pub src: SourceRefs,
    pub translations: ErrataTranslations
}

impl Identifiable for Script {
    fn id(&self) -> Id {
        Id::new(Category::Scripts, self.id)
    }
}

impl Translatable for Script {
    type Localization = ErrataLocalization;

    fn translations(&self) -> &Translations<ErrataLocalization> {
        &self.translations
    }
}
