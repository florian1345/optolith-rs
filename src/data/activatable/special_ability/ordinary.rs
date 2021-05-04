use crate::data::activatable::special_ability::{
    SimpleSpecialAbility,
    SkillInfluencingSpecialAbility,
    SpecialAbilityLocalization
};
use crate::id::{Category, CategoryProvider};

pub type OrdinarySpecialAbility<C> =
    SimpleSpecialAbility<C, SpecialAbilityLocalization>;

pub type OrdinarySkillInfluencingSpecialAbility<C> =
    SkillInfluencingSpecialAbility<C, SpecialAbilityLocalization>;

pub struct GeneralSpecialAbilityCategory;

impl CategoryProvider for GeneralSpecialAbilityCategory {
    const CATEGORY: Category = Category::GeneralSpecialAbilities;
}

pub type GeneralSpecialAbility =
    OrdinarySkillInfluencingSpecialAbility<GeneralSpecialAbilityCategory>;

pub struct FatePointSpecialAbilityCategory;

impl CategoryProvider for FatePointSpecialAbilityCategory {
    const CATEGORY: Category = Category::FatePointSpecialAbilities;
}

pub type FatePointSpecialAbility =
    OrdinarySkillInfluencingSpecialAbility<FatePointSpecialAbilityCategory>;

pub struct SexSpecialActivityCategory;

impl CategoryProvider for SexSpecialActivityCategory {
    const CATEGORY: Category = Category::SexSpecialAbilities;
}

pub type SexSpecialAbility =
    OrdinarySkillInfluencingSpecialAbility<SexSpecialActivityCategory>;

pub struct FatePointSexSpecialAbilityCategory;

impl CategoryProvider for FatePointSexSpecialAbilityCategory {
    const CATEGORY: Category = Category::FatePointSexSpecialAbilities;
}

pub type FatePointSexSpecialAbility =
    OrdinarySpecialAbility<FatePointSexSpecialAbilityCategory>;

pub struct SikaryanDrainSpecialAbilityCategory;

impl CategoryProvider for SikaryanDrainSpecialAbilityCategory {
    const CATEGORY: Category = Category::SikaryanDrainSpecialAbilities;
}

pub type SikaryanDrainSpecialAbility =
    OrdinarySpecialAbility<SikaryanDrainSpecialAbilityCategory>;
