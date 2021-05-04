use crate::data::{Translatable, Translations};
use crate::data::activatable::{
    APValue,
    SelectOptions,
    SkillApplications,
    SkillUses
};
use crate::data::activatable::special_ability::{
    AdvancedSpecialAbilities,
    SpecialAbilityLocalization
};
use crate::data::prerequisite::GeneralListOrByLevelPrerequisite;
use crate::data::skill::combat::CombatTechniqueId;
use crate::data::src::SourceRefs;
use crate::id::{Category, CategoryProvider, Id, Identifiable};

use serde::{Deserialize, Serialize};

use std::marker::PhantomData;

/// Type of combat special ability. The type id.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum CombatSpecialAbilityType {
    Passive,
    BaseManeuver,
    SpecialManeuver
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum AllCombatTechniqueRestriction {
    Improvised,
    PointedBlade,
    Mount,
    Race(u32),
    ExcludeTechniques(Vec<CombatTechniqueId>)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum AllMeleeCombatTechniqueRestriction {
    Improvised,
    PointedBlade,
    Mount,
    HasParry,
    OneHanded,
    ParryingWeapon,
    Race(u32),
    ExcludeTechniques(Vec<u32>)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum AllRangedCombatTechniqueRestriction {
    Improvised,
    PointedBlade,
    Mount,
    Race(u32),
    ExcludeTechniques(Vec<u32>)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum SpecificCombatTechniqueRestriction {
    Improvised,
    PointedBlade,
    Mount,
    Race(u32),
    Level(u32),
    Weapons(Vec<u32>)
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpecificCombatTechnique {
    pub id: CombatTechniqueId,
    pub restrictions: Option<Vec<SpecificCombatTechniqueRestriction>>
}

/// Applicable combat techniques. Specify a list of combat technique IDs if
/// only specific combat techniques are applicable, but you can also specify a
/// specific combat technique group. Leave empty if entry has no specific
/// associated combat techniques (\"–\"). 1: All; 2: All melee CTs; 3: All
/// ranged CTs; 4: All melee CTs with parry; 5: All melee CTs used with
/// one-handed weapons.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum ApplicableCombatTechniques {
    None,
    DependingOnCombatStyle,
    All(Option<Vec<AllCombatTechniqueRestriction>>),
    AllMelee(Option<Vec<AllMeleeCombatTechniqueRestriction>>),
    AllRanged(Option<Vec<AllRangedCombatTechniqueRestriction>>),
    Specific(Vec<SpecificCombatTechnique>)
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CombatSpecialAbilitySuper<C: CategoryProvider> {
    pub id: u32,
    pub levels: Option<u32>,
    pub max: Option<u32>,
    #[serde(rename = "type")]
    pub csa_type: CombatSpecialAbilityType,

    /// Registers new applications, which get enabled once this entry is
    /// activated. It specifies an entry-unique identifier and the skill it
    /// belongs to. A translation can be left out if its name equals the name
    /// of the origin entry.
    #[serde(rename = "skillApplications")]
    pub skill_applications: Option<SkillApplications>,

    /// Registers uses, which get enabled once this entry is activated. It
    /// specifies an entry-unique identifier and the skill it belongs to. A
    /// translation can be left out if its name equals the name of the origin
    /// entry.
    #[serde(rename = "skillUses")]
    pub skill_uses: Option<SkillUses>,
    #[serde(rename = "selectOptions")]
    pub select_options: Option<SelectOptions>,
    pub prerequisites: Option<GeneralListOrByLevelPrerequisite>,
    #[serde(rename = "combatTechniques")]
    pub combat_techniques: Option<ApplicableCombatTechniques>,
    #[serde(rename = "apValue")]
    pub ap_value: Option<APValue>,
    pub src: SourceRefs,
    pub translations: Translations<SpecialAbilityLocalization>,
    #[serde(skip)]
    category: PhantomData<C>
}

impl<C: CategoryProvider> Identifiable for CombatSpecialAbilitySuper<C> {
    fn id(&self) -> Id {
        Id::new(C::CATEGORY, self.id)
    }
}

impl<C> Translatable for CombatSpecialAbilitySuper<C>
where
    C: CategoryProvider
{
    type Localization = SpecialAbilityLocalization;

    fn translations(&self) -> &Translations<SpecialAbilityLocalization> {
        &self.translations
    }
}

pub struct CombatSpecialAbilityCategory;

impl CategoryProvider for CombatSpecialAbilityCategory {
    const CATEGORY: Category = Category::CombatSpecialAbilities;
}

pub type CombatSpecialAbility =
    CombatSpecialAbilitySuper<CombatSpecialAbilityCategory>;

pub struct AdvancedCombatSpecialAbilityCategory;

impl CategoryProvider for AdvancedCombatSpecialAbilityCategory {
    const CATEGORY: Category = Category::AdvancedCombatSpecialAbilities;
}

pub type AdvancedCombatSpecialAbility =
    CombatSpecialAbilitySuper<AdvancedCombatSpecialAbilityCategory>;

pub struct BrawlingSpecialAbilityCategory;

impl CategoryProvider for BrawlingSpecialAbilityCategory {
    const CATEGORY: Category = Category::BrawlingSpecialAbilities;
}

pub type BrawlingSpecialAbility =
    CombatSpecialAbilitySuper<BrawlingSpecialAbilityCategory>;

pub struct CommandSpecialAbilityCategory;

impl CategoryProvider for CommandSpecialAbilityCategory {
    const CATEGORY: Category = Category::CommandSpecialAbilities;
}

pub type CommandSpecialAbility =
    CombatSpecialAbilitySuper<CommandSpecialAbilityCategory>;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CombatStyleSpecialAbility {
    pub id: u32,
    pub levels: Option<u32>,
    pub max: Option<u32>,
    #[serde(rename = "type")]
    pub csa_type: CombatSpecialAbilityType,

    /// Is this an armed (true) or unarmed (false) combat style?
    #[serde(rename = "isArmed")]
    pub is_armed: bool,
    #[serde(rename = "selectOptions")]
    pub select_options: Option<SelectOptions>,
    pub advanced: AdvancedSpecialAbilities,
    pub prerequisites: Option<GeneralListOrByLevelPrerequisite>,
    #[serde(rename = "combatTechniques")]
    pub combat_techniques: ApplicableCombatTechniques,
    #[serde(rename = "apValue")]
    pub ap_value: Option<APValue>,
    pub src: SourceRefs,
    pub translations: Translations<SpecialAbilityLocalization>
}

impl Identifiable for CombatStyleSpecialAbility {
    fn id(&self) -> Id {
        Id::new(Category::CombatStyleSpecialAbilities, self.id)
    }
}

impl Translatable for CombatStyleSpecialAbility {
    type Localization = SpecialAbilityLocalization;

    fn translations(&self) -> &Translations<SpecialAbilityLocalization> {
        &self.translations
    }
}
