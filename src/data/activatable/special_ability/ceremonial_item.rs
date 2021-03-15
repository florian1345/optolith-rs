use crate::data::{Translatable, Translations};
use crate::data::activatable::{APValue, SelectOptions};
use crate::data::activatable::special_ability::{
    EffectSpecialAbilityLocalization
};
use crate::data::prerequisite::GeneralListOrByLevelPrerequisite;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CeremonialItemSpecialAbility {
    pub id: u32,
    pub levels: Option<u32>,
    pub max: Option<u32>,
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

impl Translatable for CeremonialItemSpecialAbility {
    type Localization = EffectSpecialAbilityLocalization;

    fn translations(&self) -> &Translations<Self::Localization> {
        &self.translations
    }
}
