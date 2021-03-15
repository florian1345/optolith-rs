use crate::data::{Translatable, Translations};
use crate::data::activatable::{ActivatableType, APValue, SelectOptions};
use crate::data::activatable::special_ability::{
    EffectSpecialAbilityLocalization,
    SimpleSpecialAbility,
    SpecialAbilityLocalization
};
use crate::data::prerequisite::GeneralListOrByLevelPrerequisite;
use crate::data::src::SourceRefs;
use crate::id::{CategoryProvider, Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

pub struct LycantropicGiftCategory;

impl CategoryProvider for LycantropicGiftCategory {
    const CATEGORY: Category = Category::LycantropicGifts;
}

pub type LycantropicGift =
    SimpleSpecialAbility<LycantropicGiftCategory, SpecialAbilityLocalization>;

pub struct VampiricGiftCategory;

impl CategoryProvider for VampiricGiftCategory {
    const CATEGORY: Category = Category::VampiricGifts;
}

pub type VampiricGift =
    SimpleSpecialAbility<VampiricGiftCategory, SpecialAbilityLocalization>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum PactGiftDemonicConsumption {
    Fixed(u32),
    PerLevel(u32)
}

#[derive(Deserialize, Serialize)]
pub enum AutomaticEntryAction {
    Add,
    Remove
}

#[derive(Deserialize, Serialize)]
pub enum AutomaticEntrySelectionTargetType {
    MagicalTraditions,
    MagicalDilettanteTraditions
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum AutomaticEntryTarget {
    Selection {
        #[serde(rename = "type")]
        target_type: AutomaticEntrySelectionTargetType
    },
    Fixed {
        #[serde(rename = "type")]
        target_type: ActivatableType,
        value: u32
    }
}

#[derive(Deserialize, Serialize)]
pub struct AutomaticEntry {

    /// What type of action is applied to the target?
    pub action: AutomaticEntryAction,

    /// If an entry is added, does is cost AP or is it free of charge?
    #[serde(rename = "noCost")]
    pub no_cost: Option<bool>,

    /// The entry that is to be added or removed. It can be a fixed entry or a
    /// selection where the player must choose one entry.
    pub target: AutomaticEntryTarget
}

#[derive(Deserialize, Serialize)]
pub struct PactGift {
    pub id: u32,
    pub levels: Option<u32>,
    pub max: Option<u32>,
    #[serde(rename = "selectOptions")]
    pub select_options: Option<SelectOptions>,

    /// This pact gift causes permanent level(s) of condition Demonic
    /// Consumption.
    #[serde(rename = "permanentDemonicConsumption")]
    pub permanent_demonic_consumption: Option<PactGiftDemonicConsumption>,

    /// This pact gift has direct influence on the existence of other entries.
    /// It may add or remove entries.
    #[serde(rename = "automaticEntries")]
    pub automatic_entries: Option<Vec<AutomaticEntry>>,
    pub prerequisites: Option<GeneralListOrByLevelPrerequisite>,
    #[serde(rename = "apValue")]
    pub ap_value: Option<APValue>,
    pub src: SourceRefs,
    pub translations: Translations<EffectSpecialAbilityLocalization>,
}

impl Identifiable for PactGift {
    fn id(&self) -> Id {
        Id::new(Category::PactGifts, self.id)
    }
}

impl Translatable for PactGift {
    type Localization = EffectSpecialAbilityLocalization;

    fn translations(&self) -> &Translations<EffectSpecialAbilityLocalization> {
        &self.translations
    }
}
