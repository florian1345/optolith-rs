use serde::{Deserialize, Serialize};

use crate::Sex;
use crate::data::{Ids, Translations};
use crate::data::activatable::{
    ActivatableId,
    ActivatableType,
    SelectOptionId
};

/// This property customizes the appearance of the prerequisite in generated
/// lists: You can hide them or replace them with a text.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum DisplayOption {
    Hide,
    ReplaceWith(Translations<String>)
}

/// Requires a specific sex.
pub type SexPrerequisite = Sex;

/// Requires a specific race or one of a specific set of races. You can also
/// provide an object to say whether the hero must meet one of the races or
/// if the entry does not allow one of the races.
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum RacePrerequisite {
    Single(u32),
    List(Vec<u32>),
    Object {
        races: Ids,
        active: bool
    }
}

/// Requires a specific culture or one of a specific set of cultures.
pub type CulturePrerequisite = Ids;

/// Requires a specific pact.
#[derive(Deserialize, Serialize)]
pub struct PactPrerequisite {

    /// The required pact category.
    pub category: u32,

    /// A specific required domain or a set of required domains.
    pub domain: Option<Ids>,

    /// The required pact level.
    pub level: Option<u32>
}

/// Requires a minimum social status.
pub type SocialStatusPrerequisite = u32;

/// Requires a specific state or one state in a set of specific states to be
/// active.
pub type StatePrerequisite = Ids;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum RuleId {
    FocusRule(u32),
    OptionalRule(u32)
}

/// Requires a specific focus or optional rule to be active.
#[derive(Deserialize, Serialize)]
pub struct RulePrerequisite {
    pub id: RuleId
}

/// Requires the primary attribute at a specific value.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum PrimaryAttributePrerequisite {
    Blessed(u32),
    Magical(u32)
}

/// Requires a specific advantage, disadvantage, special ability.
#[derive(Deserialize, Serialize)]
pub struct ActivatablePrerequisite {
    pub id: ActivatableId,
    pub active: bool,
    pub level: Option<u32>,
    pub options: Option<Vec<SelectOptionId>>
}

#[derive(Deserialize, Serialize)]
pub struct ActivatableMultiId {
    #[serde(rename = "type")]
    pub act_type: ActivatableType,
    pub value: Vec<u32>
}

/// Require one advantage, disadvantage or special ability from a set.
#[derive(Deserialize, Serialize)]
pub struct ActivatableMultiEntryPrerequisite {
    pub id: ActivatableMultiId,
    pub active: bool,
    pub level: Option<u32>,
    pub options: Option<Vec<SelectOptionId>>
}

/// Requires one of a set of options on a specific advantage, disadvantage,
/// special ability.
#[derive(Deserialize, Serialize)]
pub struct ActivatableMultiSelectPrerequisite {
    pub id: ActivatableId,

    /// If the required entry should be required to be active or inactive.
    pub active: bool,

    /// The current or required level of the entry.
    pub level: Option<u32>,

    /// Required select options. Order is important. Typically, you only need
    /// the first array index, though.
    #[serde(rename = "firstOption")]
    pub first_option: Vec<u32>,

    /// Required select options. Order is important. Typically, you only need
    /// the first array index, though.
    #[serde(rename = "otherOptions")]
    pub other_options: Vec<SelectOptionId>
}

#[derive(Deserialize, Serialize)]
pub enum IncreasableType {
    Attribute,
    Skill,
    MeleeCombatTechnique,
    RangedCombatTechnique,
    Spell,
    Ritual,
    LiturgicalChant,
    Ceremony
}

#[derive(Deserialize, Serialize)]
pub struct IncreasableId {
    #[serde(rename = "type")]
    pub inc_type: IncreasableType,
    pub value: u32
}

/// Requires a specific attribute, skill, combat technique, spell or chant to
/// be on a minimum value. Note that liturgical chants are required to be
/// active automatically, so to require them to be active you can set the value
/// to 0.
#[derive(Deserialize, Serialize)]
pub struct IncreasablePrerequisite {
    pub id: IncreasableId,
    pub value: u32
}

#[derive(Deserialize, Serialize)]
pub struct IncreasableMultiId {
    #[serde(rename = "type")]
    pub inc_type: IncreasableType,
    pub value: Vec<u32>
}

/// Require a specific attribute, skill, combat technique, spell or chant from
/// a set to be on a minimum value. Note that liturgical chants are required to
/// be active automatically, so to require them to be active you can set the
/// value to 0.
#[derive(Deserialize, Serialize)]
pub struct IncreasableMultiEntryPrerequisite {
    pub id: IncreasableMultiId,
    pub value: u32
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum GeneralPrerequisiteNoDisplay {
    Sex(SexPrerequisite),
    Race(RacePrerequisite),
    Culture(CulturePrerequisite),
    Pact(PactPrerequisite),
    SocialStatus(SocialStatusPrerequisite),
    State(StatePrerequisite),
    Rule(RulePrerequisite),
    PrimaryAttribute(PrimaryAttributePrerequisite),
    Activatable(ActivatablePrerequisite),
    ActivatableMultiEntry(ActivatableMultiEntryPrerequisite),
    ActivatableMultiSelect(ActivatableMultiSelectPrerequisite),
    Increasable(IncreasablePrerequisite),
    IncreasableMultiEntry(IncreasableMultiEntryPrerequisite),
    TraditionCanUseRituals
}

/// A prerequisite that wraps data of type `P` which is missing an optional
/// [DisplayOption] field.
#[derive(Deserialize, Serialize)]
pub struct DisplayPrerequisite<P> {
    #[serde(flatten)]
    #[serde(bound(deserialize = "P: Deserialize<'de>"))]
    pub prerequisite: P,
    #[serde(rename = "displayOption")]
    pub display_option: Option<DisplayOption>
}

pub type GeneralPrerequisite =
    DisplayPrerequisite<GeneralPrerequisiteNoDisplay>;

pub type GeneralListPrerequisite = Vec<GeneralPrerequisite>;

#[derive(Deserialize, Serialize)]
pub struct ByLevelPrerequisiteSingle<P> {
    pub level: u32,
    #[serde(bound(deserialize = "P: Deserialize<'de>"))]
    pub prerequisites: P
}

pub type GeneralByLevelPrerequisite =
    Vec<ByLevelPrerequisiteSingle<GeneralListPrerequisite>>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum ListOrByLevelPrerequisite<L, B> {
    #[serde(bound(deserialize = "L: Deserialize<'de>"))]
    Plain(L),
    #[serde(bound(deserialize = "B: Deserialize<'de>"))]
    ByLevel(B)
}

pub type GeneralListOrByLevelPrerequisite =
    ListOrByLevelPrerequisite<
        GeneralListPrerequisite,
        GeneralByLevelPrerequisite>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum TraitPrerequisiteNoDisplay {
    CommonSuggestedByRCP,
    Sex(SexPrerequisite),
    Race(RacePrerequisite),
    Culture(CulturePrerequisite),
    Pact(PactPrerequisite),
    SocialStatus(SocialStatusPrerequisite),
    State(StatePrerequisite),
    Rule(RulePrerequisite),
    PrimaryAttribute(PrimaryAttributePrerequisite),
    Activatable(ActivatablePrerequisite),
    ActivatableMultiEntry(ActivatableMultiEntryPrerequisite),
    ActivatableMultiSelect(ActivatableMultiSelectPrerequisite),
    Increasable(IncreasablePrerequisite),
    IncreasableMultiEntry(IncreasableMultiEntryPrerequisite),
    TraditionCanUseRituals
}

pub type TraitPrerequisite = DisplayPrerequisite<TraitPrerequisiteNoDisplay>;

pub type TraitListPrerequisite = Vec<TraitPrerequisite>;

pub type TraitByLevelPrerequisite =
    Vec<ByLevelPrerequisiteSingle<TraitListPrerequisite>>;

pub type TraitlListOrByLevelPrerequisite =
    ListOrByLevelPrerequisite<TraitListPrerequisite, TraitByLevelPrerequisite>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum ArcaneTraditionPrerequisiteNoDisplay {
    Sex(SexPrerequisite),
    Culture(CulturePrerequisite)
}

pub type ArcaneTraditionPrerequisite =
    DisplayPrerequisite<ArcaneTraditionPrerequisiteNoDisplay>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum ArcaneTraditionListPrerequisite {
    Plain(Vec<ArcaneTraditionPrerequisite>)
}

/// A prerequisite enumeration that only contains an increasable prerequisite.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum OnlyIncreasablePrerequisiteNoDisplay {
    Increasable(IncreasablePrerequisite)
}

pub type OnlyIncreasablePrerequisite =
    DisplayPrerequisite<OnlyIncreasablePrerequisiteNoDisplay>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum IncreasableListPrerequisite {
    Plain(Vec<OnlyIncreasablePrerequisite>)
}

/// A prerequisite enumeration that only contains an activatable prerequisite.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum OnlyActivatablePrerequisiteNoDisplay {
    Activatable(ActivatablePrerequisite)
}

pub type OnlyActivatablePrerequisite =
    DisplayPrerequisite<OnlyActivatablePrerequisiteNoDisplay>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum ActivatableListPrerequisite {
    Plain(Vec<OnlyActivatablePrerequisite>)
}

/// Require a previous enhancement.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum EnhancementPrerequisite {
    Enhancement(u32)
}

pub type EnhancementListPrerequisite = Vec<EnhancementPrerequisite>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum InfluencePrerequisiteNoDisplay {
    MagicalTradition {
        id: u32
    },
    BlessedTradition {
        id: u32
    },
    Influence {
        id: u32,
        active: bool
    },
    Special
}

pub type InfluencePrerequisite =
    DisplayPrerequisite<InfluencePrerequisiteNoDisplay>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum InfluenceListPrerequisite {
    Plain(Vec<InfluencePrerequisite>)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum ProfessionPrerequisiteNoDisplay {
    Sex(SexPrerequisite),
    Race(RacePrerequisite),
    Culture(CulturePrerequisite),
    Activatable(ActivatablePrerequisite),
    Increasable(IncreasablePrerequisite)
}

pub type ProfessionPrerequisite =
    DisplayPrerequisite<ProfessionPrerequisiteNoDisplay>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum ProfessionListPrerequisite {
    Plain(Vec<ProfessionPrerequisite>)
}
