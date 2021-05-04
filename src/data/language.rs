use crate::data::{Localization, Translatable, Translations};
use crate::data::errata::Errata;
use crate::data::prerequisite::LanguageListOrByLevelPrerequisite;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LanguageSpecializationLocalization {

    /// The name.
    pub name: String,

    /// The specialization description. It will be appended to the name in
    /// parenthesis.
    pub description: Option<String>
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LanguageSpecialization {
    pub id: u32,
    pub translations: Translations<LanguageSpecializationLocalization>
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LanguageLocalization {
    pub name: String,

    /// A list of alternative names.
    #[serde(rename = "alternativeNames")]
    pub alternative_names: Option<Vec<String>>,

    /// If specialization cannot be specified by a list, e.g. specializations
    /// for different tribes, insert the description here.
    pub specializations: Option<String>,

    /// If specialization cannot be specified by a list, e.g. specializations
    /// for different tribes, insert the description for the input field here.
    #[serde(rename = "specializationInput")]
    pub specialization_input: Option<String>,

    /// The language description.
    pub description: Option<String>,
    pub errata: Option<Errata>
}

impl Localization for LanguageLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Continent {
    pub id: u32,

    /// Is the language/script considered virtually extinct in this continent?
    #[serde(rename = "isExtinct")]
    pub is_extinct: bool
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Language {
    pub id: u32,

    /// The max level. Only specific, if lower than default of 3.
    #[serde(rename = "maxLevel")]
    pub max_level: Option<u32>,

    /// A list of language-specific specializations.
    pub specializations: Option<Vec<LanguageSpecialization>>,

    /// The continents this language is present on.
    pub continent: Vec<Continent>,
    pub prerequisites: Option<LanguageListOrByLevelPrerequisite>,
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
#[serde(deny_unknown_fields)]
pub struct AssociatedLanguage {
    pub id: u32
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScriptLocalization {

    /// The name of the script.
    pub name: String,

    /// A list of alternative names.
    #[serde(rename = "alternativeNames")]
    pub alternative_names: Option<Vec<String>>,

    /// The description of the alphabet.
    pub alphabet: String,
    pub errata: Option<Errata>
}

impl Localization for ScriptLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Script {
    pub id: u32,

    /// AP value of the script.
    #[serde(rename = "apValue")]
    pub ap_value: u32,

    /// A list of associated languages.
    #[serde(rename = "associatedLanguages")]
    pub associated_languages: Vec<AssociatedLanguage>,

    /// The continents this script is present on.
    pub continent: Vec<Continent>,
    pub src: SourceRefs,
    pub translations: Translations<ScriptLocalization>
}

impl Identifiable for Script {
    fn id(&self) -> Id {
        Id::new(Category::Scripts, self.id)
    }
}

impl Translatable for Script {
    type Localization = ScriptLocalization;

    fn translations(&self) -> &Translations<ScriptLocalization> {
        &self.translations
    }
}
