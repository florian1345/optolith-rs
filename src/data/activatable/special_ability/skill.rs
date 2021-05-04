use crate::data::{Translatable, Translations};
use crate::data::activatable::special_ability::{
    AdvancedSpecialAbilities,
    APValue,
    SelectOptions,
    SpecialAbilityLocalization
};
use crate::data::activatable::special_ability::ordinary::{
    OrdinarySkillInfluencingSpecialAbility
};
use crate::data::prerequisite::GeneralListOrByLevelPrerequisite;
use crate::data::src::SourceRefs;
use crate::id::{Category, CategoryProvider, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillStyleSpecialAbility {
    pub id: u32,
    pub levels: Option<u32>,
    pub max: Option<u32>,
    #[serde(rename = "selectOptions")]
    pub select_options: Option<SelectOptions>,

    /// The Advanced Special Abilities for the respective Style Special
    /// Ability. Sometimes, only a specific select option or a set of select
    /// options of an entry is allowed, which can be modelled by the option
    /// property. It can also be that you can choose from a set of special
    /// abilities, but then you cant specify an option.
    pub advanced: AdvancedSpecialAbilities,
    pub prerequisites: Option<GeneralListOrByLevelPrerequisite>,
    #[serde(rename = "apValue")]
    pub ap_value: Option<APValue>,
    pub src: SourceRefs,
    pub translations: Translations<SpecialAbilityLocalization>
}

impl Identifiable for SkillStyleSpecialAbility {
    fn id(&self) -> Id {
        Id::new(Category::SkillStyleSpecialAbilities, self.id)
    }
}

impl Translatable for SkillStyleSpecialAbility {
    type Localization = SpecialAbilityLocalization;

    fn translations(&self) -> &Translations<SpecialAbilityLocalization> {
        &self.translations
    }
}

pub struct AdvancedSkillSpecialAbilityCategory;

impl CategoryProvider for AdvancedSkillSpecialAbilityCategory {
    const CATEGORY: Category = Category::AdvancedSkillSpecialAbilities;
}

pub type AdvancedSkillSpecialAbility =
    OrdinarySkillInfluencingSpecialAbility<AdvancedSkillSpecialAbilityCategory>;
