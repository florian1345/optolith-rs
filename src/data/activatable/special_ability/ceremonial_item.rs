use crate::data::{TranslationsTranslatable, Translations};
use crate::data::activatable::{
    APValue,
    SelectOptions,
    SkillApplications,
    SkillUses
};
use crate::data::activatable::special_ability::{
    EffectSpecialAbilityLocalization
};
use crate::data::prerequisite::GeneralListOrByLevelPrerequisite;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CeremonialItemSpecialAbility {
    pub id: u32,
    pub levels: Option<u32>,
    pub max: Option<u32>,

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

    /// The blessed aspect. See `Liturgical Chants` for the aspects array to
    /// get the ID.
    pub aspect: Option<u32>,
    #[serde(rename = "apValue")]
    pub ap_value: Option<APValue>,
    pub prerequisites: Option<GeneralListOrByLevelPrerequisite>,
    pub src: SourceRefs,
    pub translations: Translations<EffectSpecialAbilityLocalization>
}

impl Identifiable for CeremonialItemSpecialAbility {
    fn id(&self) -> Id {
        Id::new(Category::CeremonialItemSpecialAbilities, self.id)
    }
}

impl TranslationsTranslatable for CeremonialItemSpecialAbility {
    type Localization = EffectSpecialAbilityLocalization;

    fn translations(&self) -> &Translations<Self::Localization> {
        &self.translations
    }
}
