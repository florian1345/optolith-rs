use crate::data::Translatable;
use crate::data::simple::{SimpleLocalization, SimpleTranslations};
use crate::data::skill::ImprovementCost;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum AdvantageSkillPatronPower {
    Advantage {

        /// The advantage identifier.
        id: u32,

        /// It grants a higher level of the advantage.
        level: Option<u32>,

        /// It grants a specific option of the advantage.
        option: Option<u32>
    },
    Skill {

        /// The skill identifier.
        id: u32,

        /// The value that gets added to the skill.
        value: u32
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum CombatPatronPowerId {
    Attack,
    Parry,
    RangedCombat,
    Dodge,
    DamagePoints,
    Protection
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum CombatPatronPower {
    Combat {
        id: CombatPatronPowerId,

        /// The value that gets added to the skill.
        value: u32
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum AttributePatronPower {
    Attribute {

        /// The attribute identifier.
        id: u32,

        /// The value that gets added to the attribute.
        value: u32
    }
}

pub type PatronPowers = (
    Vec<AdvantageSkillPatronPower>,
    Vec<CombatPatronPower>,
    Vec<AttributePatronPower>
);

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

    /// The patron-specific powers. Used by animist power Animal Powers I–III.
    pub powers: Option<PatronPowers>,

    /// The patron-specific AE cost. Used by several animist forces.
    pub cost: Option<u32>,

    /// "The patron-specific improvement cost. Used by several animist forces.
    pub ic: Option<ImprovementCost>,
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
#[serde(deny_unknown_fields)]
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
