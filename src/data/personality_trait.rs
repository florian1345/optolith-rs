use crate::data::{Translatable, Translations};
use crate::data::effects_localization::EffectsLocalization;
use crate::data::prerequisite::PersonalityTraitListPrerequisite;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PersonalityTrait {
    pub id: u32,

    /// The personality trait's level.
    pub level: u32,
    pub prerequisites: Option<PersonalityTraitListPrerequisite>,

    /// The lower-level personality trait(s) this trait can be combined with.
    #[serde(rename = "canBeCombinedWith")]
    pub can_be_combined_with: Option<Vec<u32>>,
    pub src: SourceRefs,
    pub translations: Translations<EffectsLocalization>
}

impl Identifiable for PersonalityTrait {
    fn id(&self) -> Id {
        Id::new(Category::PersonalityTraits, self.id)
    }
}

impl Translatable for PersonalityTrait {
    type Localization = EffectsLocalization;

    fn translations(&self) -> &Translations<EffectsLocalization> {
        &self.translations
    }
}
