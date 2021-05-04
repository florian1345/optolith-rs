use crate::data::{Localization, Translatable, Translations};
use crate::data::errata::Errata;
use crate::data::skill::ImprovementCost;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CombatTechniqueLocalization {
    pub name: String,

    /// Special rules for the respective combat technique – if there are any.
    /// Markdown available.
    pub special: Option<String>,
    pub errata: Option<Errata>
}

impl Localization for CombatTechniqueLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MeleeCombatTechnique {
    pub id: u32,

    /// Improvement cost.
    pub ic: ImprovementCost,

    /// The primary attribute(s).
    pub primary: Vec<u32>,

    /// The Breaking Point Rating of the respective combat technique.
    #[serde(rename = "breakingPointRating")]
    pub breaking_point_rating: u32,

    /// Is parry forbidden?
    #[serde(rename = "hasNoParry")]
    pub has_no_parry: bool,
    pub src: SourceRefs,
    pub translations: Translations<CombatTechniqueLocalization>
}

impl Identifiable for MeleeCombatTechnique {
    fn id(&self) -> Id {
        Id::new(Category::MeleeCombatTechniques, self.id)
    }
}

impl Translatable for MeleeCombatTechnique {
    type Localization = CombatTechniqueLocalization;

    fn translations(&self) -> &Translations<CombatTechniqueLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RangedCombatTechnique {
    pub id: u32,

    /// Improvement cost.
    pub ic: ImprovementCost,

    /// The primary attribute(s).
    pub primary: Vec<u32>,

    /// The Breaking Point Rating of the respective combat technique.
    #[serde(rename = "breakingPointRating")]
    pub breaking_point_rating: u32,
    pub src: SourceRefs,
    pub translations: Translations<CombatTechniqueLocalization>
}

impl Identifiable for RangedCombatTechnique {
    fn id(&self) -> Id {
        Id::new(Category::RangedCombatTechniques, self.id)
    }
}

impl Translatable for RangedCombatTechnique {
    type Localization = CombatTechniqueLocalization;

    fn translations(&self) -> &Translations<CombatTechniqueLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum CombatTechniqueId {
    MeleeCombatTechnique(u32),
    RangedCombatTechnique(u32)
}
