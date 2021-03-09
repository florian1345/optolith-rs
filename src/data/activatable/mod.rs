use crate::data::{Category, Localization, Translations};
use crate::data::errata::Errata;
use crate::data::prerequisite::GeneralListOrByLevelPrerequisite;
use crate::data::src::SourceRefs;

use serde::{Deserialize, Serialize};

pub mod character_trait;
pub mod special_ability;

#[derive(Deserialize, Serialize)]
pub enum ActivatableType {
    Advantage,
    Disadvantage,
    GeneralSpecialAbility,
    FatePointSpecialAbility,
    CombatSpecialAbility,
    MagicalSpecialAbility,
    StaffEnchantment,
    FamiliarSpecialAbility,
    KarmaSpecialAbility,
    ProtectiveWardingCircleSpecialAbility,
    CombatStyleSpecialAbility,
    AdvancedCombatSpecialAbility,
    CommandSpecialAbility,
    MagicStyleSpecialAbility,
    AdvancedMagicalSpecialAbility,
    SpellSwordEnchantment,
    DaggerRitual,
    InstrumentEnchantment,
    AttireEnchantment,
    OrbEnchantment,
    WandEnchantment,
    BrawlingSpecialAbility,
    AncestorGlyph,
    CeremonialItemSpecialAbility,
    Sermon,
    LiturgicalStyleSpecialAbility,
    AdvancedKarmaSpecialAbility,
    Vision,
    MagicalTradition,
    BlessedTradition,
    PactGift,
    SikaryanDrainSpecialAbility,
    LykanthropicGift,
    Talentstilsonderfertigkeit,
    AdvancedSkillSpecialAbility,
    ArcaneOrbEnchantment,
    CauldronEnchantment,
    FoolsHatEnchantment,
    ToyEnchantment,
    BowlEnchantment,
    FatePointSexSpecialAbility,
    SexSpecialAbility,
    WeaponEnchantment,
    SickleRitual,
    RingEnchantment,
    ChronicleEnchantment
}

#[derive(Deserialize, Serialize)]
pub struct ActivatableId {
    #[serde(rename = "type")]
    pub act_type: ActivatableType,
    pub value: u32
}

#[derive(Deserialize, Serialize)]
pub struct CategorySelectOptionPrerequisite {
    pub target: ActivatableId,
    pub active: bool,
    pub level: Option<u32>
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum CategoryPrerequisite {

    /// The category entry requires or prohibits itself as a select option of
    /// another entry.
    SelectOption(CategorySelectOptionPrerequisite),

    /// The category entry requires itself on a certain Skill Rating.
    #[serde(rename = "Self")]
    SelfRating(u32)
}

#[derive(Deserialize, Serialize)]
pub struct DerivedFromAbilityEntryAPValue {
    pub id: u32,
    #[serde(rename = "apValue")]
    pub ap_value: u32
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum DerivedFromAbilityAPValue {

    /// The wrapped number is multiplied with the improvement cost of the entry
    /// (A = 1 to D = 4).
    DerivedFromIC(u32),

    Fixed {

        /// A mapping of entry ids to their specific AP values.
        list: Vec<DerivedFromAbilityEntryAPValue>,

        /// The default value of an entry. Used as a fallback if no value is
        /// found in `list`.
        default: u32
    }
}

/// Entries of the list of category IDs of derived select options.
#[derive(Deserialize, Serialize)]
pub struct DerivedSelectOption {
    pub category: Category,

    /// Only include entries with the listed ids.
    pub specific: Option<Vec<u32>>,

    /// Does each aspect/property require it's corresponding
    /// aspect/property knowledge? (Only for category Aspects/Properties)
    #[serde(rename = "requireKnowledge")]
    pub require_knowledge: Option<bool>,

    /// The generated name should be the Master of (Aspect) suffix for this
    /// aspect instead of the aspect's name. If an aspect does not provide
    /// a suffix (such as the General aspect), it is automatically excluded
    /// from the list. (Only for category Aspects)
    #[serde(rename = "useMasterOfSuffixAsName")]
    pub use_master_of_suffix_as_name: Option<bool>,

    /// Only convert half the entry level into the AP value. (Only for
    /// category Diseases/Poisons)
    #[serde(rename = "useHalfLevelAsApValue")]
    pub use_half_level_as_ap_value: Option<bool>,

    /// Generate prerequisites for each entry of the category. (Only for
    /// category Languages/Skills/MeleeCombatTechniques/
    /// RangedCombatTechniques/LiturgicalChants/Ceremonies/Spells/Rituals)
    pub prerequisites: Option<Vec<CategoryPrerequisite>>,

    /// Only include entries of the specified groups. (Only for category
    /// Skills)
    pub groups: Option<Vec<u32>>,

    /// Exclude entries with the listed ids. This has no effect if
    /// `specific` is used.
    pub exclude: Option<Vec<u32>>,

    /// Generate AP values for each entry. (Only for category Skills/
    /// MeleeCombatTechniques/RangedCombatTechniques/LiturgicalChants/
    /// Ceremonies/Spells/Rituals)
    #[serde(rename = "apValue")]
    pub ap_value: Option<DerivedFromAbilityAPValue>
}

#[derive(Deserialize, Serialize)]
pub enum SelectOptionAbility {
    Skill,
    MeleeCombatTechnique,
    RangedCombatTechnique
}

/// An identification of a select option for a specific activatable, if the
/// activatable is apparent from context.
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum SelectOptionId {
    Integer(i32),
    Ability {
        #[serde(rename = "type")]
        ability_type: SelectOptionAbility,
        value: u32
    }
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ExplicitSelectOptionLocalization {
    Ordinary {
        name: String,
        description: Option<String>,
        errata: Option<Errata>
    },
    ErrataOnly {
        errata: Option<Errata>
    }
}

impl Localization for ExplicitSelectOptionLocalization {
    fn name(&self) -> &str {
        match self {
            ExplicitSelectOptionLocalization::Ordinary {
                name,
                description: _,
                errata: _
            } => {
                name
            },
            ExplicitSelectOptionLocalization::ErrataOnly { errata: _ } => {
                // TODO resolve
                "UNKNOWN_SELECT_OPTION"
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ExplicitSelectOption {
    pub id: SelectOptionId,
    pub prerequisites: Option<GeneralListOrByLevelPrerequisite>,

    /// Specific AP cost for the select option.
    #[serde(rename = "apValue")]
    pub ap_value: Option<u32>,
    pub src: Option<SourceRefs>,
    pub translations: Option<Translations<ExplicitSelectOptionLocalization>>
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum SelectOptions {

    /// A list of category ids. All available entries from the specified
    /// category/ies will be included as separate select options. You can also
    /// specify a set of groups that should only be included. Groups not
    /// mentioned will be excluded then.
    Derived(Vec<DerivedSelectOption>),

    /// A list of explicit select options. If the id has a specific type, its
    /// entry is the base of this select option, where values defined here
    /// override values from the base. Define the `src` property if the options
    /// are not derived from the rules text of the
    /// advantage/disadvantage/special ability but instead are listed in a
    /// separate block and/or on a separate page.
    Explicit(Vec<ExplicitSelectOption>)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum APValue {
    Flat(u32),
    PerLevel(Vec<u32>),
    /// Used if AP value is defined by the selected option(s).
    Option
}
