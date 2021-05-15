use crate::data::{Category, Ids, Localization, Translations};
use crate::data::errata::Errata;
use crate::data::prerequisite::GeneralListOrByLevelPrerequisite;
use crate::data::simple::SimpleTranslations;
use crate::data::src::SourceRefs;

use serde::{Deserialize, Serialize};

pub mod character_trait;
pub mod special_ability;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
    LycantropicGift,
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

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActivatableId {
    #[serde(rename = "type")]
    pub act_type: ActivatableType,
    pub value: u32
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CategorySelectOptionPrerequisite {
    pub target: ActivatableId,
    pub active: bool,
    pub level: Option<u32>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum CategoryPrerequisite {

    /// The category entry requires or prohibits itself as a select option of
    /// another entry.
    SelectOption(CategorySelectOptionPrerequisite),

    /// The category entry requires itself on a certain Skill Rating.
    #[serde(rename = "Self")]
    SelfRating(u32)
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DerivedFromAbilityEntryAPValue {
    pub id: u32,
    #[serde(rename = "apValue")]
    pub ap_value: u32
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
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

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DerivedSelectOptionSkillMod {

    /// An unique, increasing integer, identifying the application in the entry
    /// it is registered for.
    pub id: u32,
    pub translations: Option<SimpleTranslations>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RequiredSkillRating {

    /// The minimum number of skills that need to be on the defined minimum
    /// skill rating.
    pub number: u32,

    /// The minimum skill rating the defined minimum number of skills need to
    /// be on.
    pub value: u32
}

/// Entries of the list of category IDs of derived select options.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DerivedSelectOption {
    pub category: Category,

    /// Only include entries with the listed ids.
    pub specific: Option<Vec<u32>>,

    /// Does each aspect/property require it's corresponding
    /// aspect/property knowledge? (Only for category Aspects/Properties)
    #[serde(rename = "requireKnowledge")]
    pub require_knowledge: Option<bool>,

    #[serde(rename = "requireSkillRating")]
    pub require_skill_rating: Option<RequiredSkillRating>,

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

    /// Should the principles (code) of the tradition be required to select the
    /// respective tradition? (Only for category BlessedTraditions)
    #[serde(rename = "requirePrinciples")]
    pub require_principles: Option<bool>,

    /// Registers new applications, which get enabled once this entry is
    /// activated with its respective select option. It specifies an
    /// entry-unique identifier, the skill it belongs to is derived from the
    /// select option automatically. A translation can be left out if its name
    /// equals the name of the origin entry.
    #[serde(rename = "skillApplications")]
    pub skill_applications: Option<Vec<DerivedSelectOptionSkillMod>>,

    /// Registers uses, which get enabled once this entry is activated with its
    /// respective select option. It specifies an entry-unique identifier, the
    /// skill it belongs to is derived from the select option automatically. A
    /// translation can be left out if its name equals the name of the origin
    /// entry.
    #[serde(rename = "skillUses")]
    pub skill_uses: Option<Vec<DerivedSelectOptionSkillMod>>,

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

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum SelectOptionAbility {
    Skill,
    MeleeCombatTechnique,
    RangedCombatTechnique
}

/// An identification of a select option for a specific activatable, if the
/// activatable is apparent from context.
#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum SelectOptionId {
    Integer(i32),
    Ability {
        #[serde(rename = "type")]
        ability_type: SelectOptionAbility,
        value: u32
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum ExplicitSelectOptionLocalization {
    Ordinary {
        name: String,

        /// The name of the select option when displayed in a generated
        /// profession text.
        #[serde(rename = "nameInProfession")]
        name_in_profession: Option<String>,

        /// The description of the select option. Useful for Bad Habits, Trade
        /// Secrets and other entries where a description is available.
        /// Markdown is available.
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
            ExplicitSelectOptionLocalization::Ordinary { name, .. } => {
                name
            },
            ExplicitSelectOptionLocalization::ErrataOnly { errata: _ } => {
                // TODO resolve
                "UNKNOWN_SELECT_OPTION"
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExplicitSelectOption {
    pub id: SelectOptionId,

    /// Sometimes, professions use specific text selections that are not
    /// contained in described lists. This ensures you can use them for
    /// professions only. They are not going to be displayed as options to the
    /// user.
    #[serde(rename = "professionOnly")]
    pub profession_only: Option<bool>,
    pub prerequisites: Option<GeneralListOrByLevelPrerequisite>,

    /// Registers new applications, which get enabled once this entry is
    /// activated with its respective select option. It specifies an
    /// entry-unique identifier and the skill it belongs to. A translation can
    /// be left out if its name equals the name of the origin select option.
    #[serde(rename = "skillApplications")]
    pub skill_applications: Option<SkillApplications>,

    /// Registers uses, which get enabled once this entry is activated with its
    /// respective select option. It specifies an entry-unique identifier and
    /// the skill it belongs to. A translation can be left out if its name
    /// equals the name of the origin select option.
    #[serde(rename = "skillUses")]
    pub skill_uses: Option<SkillUses>,

    /// Specific AP cost for the select option.
    #[serde(rename = "apValue")]
    pub ap_value: Option<u32>,
    pub src: Option<SourceRefs>,
    pub translations: Option<Translations<ExplicitSelectOptionLocalization>>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
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

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum APValue {
    Flat(u32),
    PerLevel(Vec<u32>),
    /// Used if AP value is defined by the selected option(s).
    Option
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillApplication {

    /// An unique, increasing integer, identifying the application in the entry
    /// it is registered for.
    pub id: u32,

    /// The identifiers of the skills this application is for.
    #[serde(rename = "skillId")]
    pub skill_id: Ids,

    /// If an application applies to multiple skills, it may need to ensure the
    /// respective skill is on a certain skill rating.
    #[serde(rename = "requiredSkillRating")]
    pub required_skill_rating: Option<u32>,
    pub translations: Option<SimpleTranslations>
}

pub type SkillApplications = Vec<SkillApplication>;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillUse {

    /// An unique, increasing integer, identifying the use in the entry it is
    /// registered for.
    pub id: u32,

    /// The identifiers of the skills this application is for.
    #[serde(rename = "skillId")]
    pub skill_id: Ids,
    pub translations: Option<SimpleTranslations>
}

pub type SkillUses = Vec<SkillUse>;
