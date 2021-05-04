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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub enum RuleId {
    FocusRule(u32),
    OptionalRule(u32)
}

/// Requires a specific focus or optional rule to be active.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RulePrerequisite {
    pub id: RuleId
}

/// Requires the primary attribute at a specific value.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum PrimaryAttributePrerequisite {
    Blessed(u32),
    Magical(u32)
}

/// Requires a specific advantage, disadvantage, special ability.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActivatablePrerequisite {
    pub id: ActivatableId,
    pub active: bool,
    pub level: Option<u32>,
    pub options: Option<Vec<SelectOptionId>>
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActivatableMultiId {
    #[serde(rename = "type")]
    pub act_type: ActivatableType,
    pub value: Vec<u32>
}

/// Require one advantage, disadvantage or special ability from a set.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActivatableMultiEntryPrerequisite {
    pub id: ActivatableMultiId,
    pub active: bool,
    pub level: Option<u32>,
    pub options: Option<Vec<SelectOptionId>>
}

/// Requires one of a set of options on a specific advantage, disadvantage,
/// special ability.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct IncreasablePrerequisite {
    pub id: IncreasableId,
    pub value: u32
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct IncreasableMultiEntryPrerequisite {
    pub id: IncreasableMultiId,
    pub value: u32
}

/// Requires a specific publication to be active.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationPrerequisite {
    pub id: u32
}

/// The `when` property defines that the prerequisite it is defined for only
/// takes effect if the prerequisites in this list are matched.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum WhenSingle {
    Publication(PublicationPrerequisite)
}

pub type When = Vec<WhenSingle>;

// TODO add deny_unknown_fields to this, TraitPrerequisiteNoDisplayNoWhen,
// DisplayPrerequisite and WhenPrerequisite once
// https://github.com/serde-rs/serde/issues/1547 is fixed.

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum GeneralPrerequisiteNoDisplayNoWhen {
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
    pub prerequisite: P,
    #[serde(rename = "displayOption")]
    pub display_option: Option<DisplayOption>
}

/// A prerequisite that wraps data of type `P` which is missing an optional
/// [When] field.
#[derive(Deserialize, Serialize)]
pub struct WhenPrerequisite<P> {
    #[serde(flatten)]
    pub prerequisite: P,
    pub when: Option<When>
}

pub type GeneralPrerequisite =
    WhenPrerequisite<DisplayPrerequisite<GeneralPrerequisiteNoDisplayNoWhen>>;

pub type GeneralListPrerequisite = Vec<GeneralPrerequisite>;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ByLevelPrerequisiteSingle<P> {
    pub level: u32,
    pub prerequisites: P
}

pub type GeneralByLevelPrerequisite =
    Vec<ByLevelPrerequisiteSingle<GeneralListPrerequisite>>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum ListOrByLevelPrerequisite<L, B> {
    Plain(L),
    ByLevel(B)
}

pub type GeneralListOrByLevelPrerequisite =
    ListOrByLevelPrerequisite<
        GeneralListPrerequisite,
        GeneralByLevelPrerequisite>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum TraitPrerequisiteNoDisplayNoWhen {
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

pub type TraitPrerequisite =
    WhenPrerequisite<DisplayPrerequisite<TraitPrerequisiteNoDisplayNoWhen>>;

pub type TraitListPrerequisite = Vec<TraitPrerequisite>;

pub type TraitByLevelPrerequisite =
    Vec<ByLevelPrerequisiteSingle<TraitListPrerequisite>>;

pub type TraitlListOrByLevelPrerequisite =
    ListOrByLevelPrerequisite<TraitListPrerequisite, TraitByLevelPrerequisite>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum ArcaneTraditionPrerequisiteNoDisplay {
    Sex(SexPrerequisite),
    Culture(CulturePrerequisite)
}

pub type ArcaneTraditionPrerequisite =
    DisplayPrerequisite<ArcaneTraditionPrerequisiteNoDisplay>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum ArcaneTraditionListPrerequisite {
    Plain(Vec<ArcaneTraditionPrerequisite>)
}

/// A prerequisite enumeration that only contains an increasable prerequisite.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum OnlyIncreasablePrerequisiteNoDisplay {
    Increasable(IncreasablePrerequisite)
}

pub type OnlyIncreasablePrerequisite =
    DisplayPrerequisite<OnlyIncreasablePrerequisiteNoDisplay>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum IncreasableListPrerequisite {
    Plain(Vec<OnlyIncreasablePrerequisite>)
}

/// A prerequisite enumeration that only contains an activatable prerequisite.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum OnlyActivatablePrerequisiteNoDisplay {
    Activatable(ActivatablePrerequisite)
}

pub type OnlyActivatablePrerequisite =
    DisplayPrerequisite<OnlyActivatablePrerequisiteNoDisplay>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum ActivatableListPrerequisite {
    Plain(Vec<OnlyActivatablePrerequisite>)
}

/// Require a previous enhancement.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum EnhancementPrerequisite {
    Enhancement(u32)
}

pub type EnhancementListPrerequisite = Vec<EnhancementPrerequisite>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub enum InfluenceListPrerequisite {
    Plain(Vec<InfluencePrerequisite>)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub enum ProfessionListPrerequisite {
    Plain(Vec<ProfessionPrerequisite>)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum PersonalityTraitPrerequisiteNoDisplay {
    Culture(CulturePrerequisite),
    Special
}

pub type PersonalityTraitPrerequisite =
    DisplayPrerequisite<PersonalityTraitPrerequisiteNoDisplay>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum PersonalityTraitListPrerequisite {
    Plain(Vec<PersonalityTraitPrerequisite>)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum LiturgicalChantPrerequisite {
    Rule(RulePrerequisite)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum LiturgicalChantListPrerequisite {
    Plain(Vec<LiturgicalChantPrerequisite>)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum LanguagePrerequisiteNoDisplay {
    Race(RacePrerequisite),
    Activatable(ActivatablePrerequisite)
}

pub type LanguagePrerequisite =
    DisplayPrerequisite<LanguagePrerequisiteNoDisplay>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum IneligiblePrerequisiteNoDisplay {
    Ineligible
}

pub type IneligiblePrerequisite =
    DisplayPrerequisite<IneligiblePrerequisiteNoDisplay>;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum LanguageListPrerequisite {
    Language(Vec<LanguagePrerequisite>),
    Ineligible(Vec<IneligiblePrerequisite>)
}

pub type LanguageByLevelPrerequisite =
    Vec<ByLevelPrerequisiteSingle<LanguageListPrerequisite>>;

pub type LanguageListOrByLevelPrerequisite =
    ListOrByLevelPrerequisite<LanguageListPrerequisite,
        LanguageByLevelPrerequisite>;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum MagicalTraditionPrerequisite {
    General(GeneralListOrByLevelPrerequisite),
    Ineligible(IneligiblePrerequisite)
}
