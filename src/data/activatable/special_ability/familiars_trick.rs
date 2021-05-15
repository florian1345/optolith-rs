use crate::data::{Localization, TranslationsTranslatable, Translations};
use crate::data::errata::Errata;
use crate::data::skill::non_profane::MainParameterLocalization;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum FamiliarAPValue {
    Flat(u32),
    Default
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FamiliarsTrickLocalization {

    /// The name of the spell.
    pub name: String,

    /// The effect description. Markdown is available.
    pub effect: String,

    /// The AE cost.
    pub cost: MainParameterLocalization,

    /// The duration.
    pub duration: MainParameterLocalization,
    pub errata: Option<Errata>
}

impl Localization for FamiliarsTrickLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FamiliarsTrick {
    pub id: u32,

    /// A list of animal types this familiar's trick is available for. Leave
    /// undefined if it is available for all types of familiars.
    #[serde(rename = "animalTypes")]
    pub animal_types: Option<Vec<u32>>,

    // The property ID.
    pub property: u32,

    /// The AP value the familiar has to pay for. It may also be that a
    /// specific is known by all familiar by default.
    #[serde(rename = "apValue")]
    pub ap_value: FamiliarAPValue,
    pub src: SourceRefs,
    pub translations: Translations<FamiliarsTrickLocalization>
}

impl Identifiable for FamiliarsTrick {
    fn id(&self) -> Id {
        Id::new(Category::FamiliarsTricks, self.id)
    }
}

impl TranslationsTranslatable for FamiliarsTrick {
    type Localization = FamiliarsTrickLocalization;

    fn translations(&self) -> &Translations<FamiliarsTrickLocalization> {
        &self.translations
    }
}
