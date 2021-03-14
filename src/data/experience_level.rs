use crate::data::{SimpleLocalization, SimpleTranslations, Translatable};
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ExperienceLevel {
    pub id: u32,

    /// The AP value you get when creating a character with this level.
    pub ap: u32,

    /// The highest possible attribute value.
    #[serde(rename = "maxAttributeValue")]
    pub max_attribute_value: u32,

    /// The highest possible skill rating.
    #[serde(rename = "maxSkillRating")]
    pub max_skill_rating: u32,

    /// The highest possible combat technique rating.
    #[serde(rename = "maxCombatTechniqueRating")]
    pub max_combat_technique_rating: u32,

    /// The limit for the sum of all attribute values.
    #[serde(rename = "maxAttributeTotal")]
    pub max_attribute_total: u32,

    /// The maximum of spells/chants you can activate.
    #[serde(rename = "maxNumberSpellsLiturgicalChants")]
    pub max_number_spells_liturgical_chants: u32,

    /// The maximum of spells of an unfamiliar tradition you can activate.
    #[serde(rename = "maxUnfamiliarSpells")]
    pub max_unfamiliar_spells: u32,
    pub translations: SimpleTranslations
}

impl Identifiable for ExperienceLevel {
    fn id(&self) -> Id {
        Id::new(Category::ExperienceLevels, self.id)
    }
}

impl Translatable for ExperienceLevel {
    type Localization = SimpleLocalization;

    fn translations(&self) -> &SimpleTranslations {
        &self.translations
    }
}
