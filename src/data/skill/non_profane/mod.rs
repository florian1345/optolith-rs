use crate::data::{Localization, TranslationsTranslatable, Translations};
use crate::data::errata::Errata;
use crate::data::prerequisite::EnhancementListPrerequisite;
use crate::data::src::SourceRefs;

use serde::{Deserialize, Serialize};

pub mod karmal;
pub mod magical;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum CheckModTarget {
    Creature
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum SimpleCheckMod {
    #[serde(rename = "SPI")]
    Spirit,
    #[serde(rename = "SPI/2")]
    HalfSpirit,
    #[serde(rename = "TOU")]
    Toughness,
    #[serde(rename = "SPI/TOU")]
    Higher,
    CreationDifficulty,
    SummoningDifficulty
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckModOf {
    #[serde(rename = "type")]
    pub simple: SimpleCheckMod,
    pub of: CheckModTarget
}

/// If the check will be modified by Spirit or Toughness, insert `SPI` or `TOU`
/// respectively. If the higher is the characteristic to choose, insert an
/// array with both instead.
#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum CheckMod {
    Simple(SimpleCheckMod),
    Of(CheckModOf)
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EnhancementLocalization {
    pub name: String,
    pub effect: String,
    pub errata: Option<Errata>
}

impl Localization for EnhancementLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Enhancement {
    pub id: u32,

    /// The level of the enhancement, which states on which SR you can buy it.
    /// Level 1 is SR 8, level 2 is SR 12 and level 3 is SR 16. The AP value is
    /// also derived from the level by multiplying the level with the numeric
    /// representation of the improvement cost of the main entry.
    pub level: Option<u32>,
    pub prerequisites: Option<EnhancementListPrerequisite>,
    pub src: Option<SourceRefs>,
    pub translations: Translations<EnhancementLocalization>
}

impl TranslationsTranslatable for Enhancement {
    type Localization = EnhancementLocalization;

    fn translations(&self) -> &Translations<EnhancementLocalization> {
        &self.translations
    }
}

pub type Enhancements = Vec<Enhancement>;

/// Effect descriptions for a reached QL. You can set an effect for each QL or
/// for each 2 QL.
#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum QualityLevelEffectLocalization {

    /// Gives an effect description for every quality level (first is QL 1,
    /// second is QL 2 etc). Markdown is available.
    Every([String; 6]),

    /// Gives an effect description for every pair of quality levels (first
    /// entry is for QL 1-2, second for QL 3-4 etc). Markdown is available.
    Pairs([String; 3])
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MainParameterLocalization {

    /// The full parameter text.
    pub full: String,

    /// The abbreviated parameter text for the character sheet.
    pub abbr: String
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NonProfaneSkillLocalization {

    /// The name of the spell/ritual/liturgy/ceremony.
    pub name: String,

    /// A short name for the spell/ritual/liturgy/ceremony used for character
    /// sheets.
    #[serde(rename = "nameShort")]
    pub name_short: Option<String>,

    /// The effect description. Markdown is available. If the effect is
    /// different for different quality levels, use `effectQualityLevels`. If
    /// there is general effect text after the list of quality levels, use
    /// `effectAfterQualityLevels` for that.
    pub effect: String,
    #[serde(rename = "effectQualityLevels")]
    pub effect_quality_levels: Option<QualityLevelEffectLocalization>,

    /// The effect description after the quality levels list. Markdown is
    /// available.
    #[serde(rename = "effectAfterQualityLevels")]
    pub effect_after_quality_levels: Option<String>,

    /// The casting/chanting/ritual/ceremony time. Markdown is available.
    #[serde(rename = "castingTime")]
    pub casting_time: MainParameterLocalization,

    /// The AE/KE cost.
    pub cost: MainParameterLocalization,

    /// The range.
    pub range: MainParameterLocalization,

    /// The duration.
    pub duration: MainParameterLocalization,

    /// The target category.
    pub target: String,
    pub errata: Option<Errata>
}

impl Localization for NonProfaneSkillLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

/// A localization for blessings and cantrips.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SmallNonProfaneSkillLocalization {

    /// The name of the cantrip/blessing.
    pub name: String,

    /// The effect description. Markdown is available.
    pub effect: String,

    /// The range.
    pub range: String,

    /// The duration.
    pub duration: String,

    /// The target category.
    pub target: String,

    /// A note, usually on the cantrips usage.
    pub note: Option<String>,
    pub errata: Option<Errata>
}

impl Localization for SmallNonProfaneSkillLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}
