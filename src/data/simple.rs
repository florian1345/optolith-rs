use crate::data::{Localization, Translations, TranslationsTranslatable};
use crate::id::{Category, CategoryProvider, Id, Identifiable};

use serde::{Deserialize, Serialize};

use std::marker::PhantomData;

/// A [Localization] that consists only of a string.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SimpleLocalization {
    pub name: String
}

impl Localization for SimpleLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

pub type SimpleTranslations = Translations<SimpleLocalization>;

/// A data entity that consists of an ID and
/// [SimpleTranslations](crate::data::SimpleTranslations).
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SimpleEntity<C: CategoryProvider> {
    pub id: u32,
    pub translations: SimpleTranslations,
    #[serde(skip)]
    c: PhantomData<C>
}

impl<C: CategoryProvider> Identifiable for SimpleEntity<C> {
    fn id(&self) -> Id {
        Id::new(C::CATEGORY, self.id)
    }
}

impl<C> TranslationsTranslatable for SimpleEntity<C>
where
    C: CategoryProvider
{
    type Localization = SimpleLocalization;

    fn translations(&self) -> &SimpleTranslations {
        &self.translations
    }
}

#[derive(Clone)]
pub struct AnimalTypeCategory;

impl CategoryProvider for AnimalTypeCategory {
    const CATEGORY: Category = Category::AnimalTypes;
}

pub type AnimalType = SimpleEntity<AnimalTypeCategory>;

#[derive(Clone)]
pub struct ArmorTypeCategory;

impl CategoryProvider for ArmorTypeCategory {
    const CATEGORY: Category = Category::ArmorTypes;
}

pub type ArmorType = SimpleEntity<ArmorTypeCategory>;

#[derive(Clone)]
pub struct BrewCategory;

impl CategoryProvider for BrewCategory {
    const CATEGORY: Category = Category::Brews;
}

pub type Brew = SimpleEntity<BrewCategory>;

#[derive(Clone)]
pub struct CombatSpecialAbilityGroupCategory;

impl CategoryProvider for CombatSpecialAbilityGroupCategory {
    const CATEGORY: Category = Category::CombatSpecialAbilityGroups;
}

pub type CombatSpecialAbilityGroup =
    SimpleEntity<CombatSpecialAbilityGroupCategory>;

    #[derive(Clone)]
pub struct CombatTechniqueGroupCategory;

impl CategoryProvider for CombatTechniqueGroupCategory {
    const CATEGORY: Category = Category::CombatTechniqueGroups;
}

pub type CombatTechniqueGroup = SimpleEntity<CombatTechniqueGroupCategory>;

#[derive(Clone)]
pub struct ElementCategory;

impl CategoryProvider for ElementCategory {
    const CATEGORY: Category = Category::Elements;
}

pub type Element = SimpleEntity<ElementCategory>;

#[derive(Clone)]
pub struct EyeColorCategory;

impl CategoryProvider for EyeColorCategory {
    const CATEGORY: Category = Category::EyeColors;
}

pub type EyeColor = SimpleEntity<EyeColorCategory>;

#[derive(Clone)]
pub struct HairColorCategory;

impl CategoryProvider for HairColorCategory {
    const CATEGORY: Category = Category::HairColors;
}

pub type HairColor = SimpleEntity<HairColorCategory>;

#[derive(Clone)]
pub struct LiturgicalChantGroupCategory;

impl CategoryProvider for LiturgicalChantGroupCategory {
    const CATEGORY: Category = Category::LiturgicalChantGroups;
}

pub type LiturgicalChantGroup = SimpleEntity<LiturgicalChantGroupCategory>;

#[derive(Clone)]
pub struct ReachCategory;

impl CategoryProvider for ReachCategory {
    const CATEGORY: Category = Category::Reaches;
}

pub type Reach = SimpleEntity<ReachCategory>;

#[derive(Clone)]
pub struct RegionCategory;

impl CategoryProvider for RegionCategory {
    const CATEGORY: Category = Category::Regions;
}

pub type Region = SimpleEntity<RegionCategory>;

#[derive(Clone)]
pub struct SocialStatusCategory;

impl CategoryProvider for SocialStatusCategory {
    const CATEGORY: Category = Category::SocialStatuses;
}

pub type SocialStatus = SimpleEntity<SocialStatusCategory>;

#[derive(Clone)]
pub struct SpellGroupCategory;

impl CategoryProvider for SpellGroupCategory {
    const CATEGORY: Category = Category::SpellGroups;
}

pub type SpellGroup = SimpleEntity<SpellGroupCategory>;

#[derive(Clone)]
pub struct SubjectCategory;

impl CategoryProvider for SubjectCategory {
    const CATEGORY: Category = Category::Subjects;
}

/// Subjects or Subject Areas are the categories of Focus Rules.
pub type Subject = SimpleEntity<SubjectCategory>;

#[derive(Clone)]
pub struct TribeCategory;

impl CategoryProvider for TribeCategory {
    const CATEGORY: Category = Category::Tribes;
}

pub type Tribe = SimpleEntity<TribeCategory>;
