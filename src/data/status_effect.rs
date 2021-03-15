use crate::data::{Localization, Translations, Translatable};
use crate::data::errata::{Errata, ErrataLocalization, ErrataTranslations};
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct ConditionLocalization {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "levelDescription")]
    pub level_description: Option<String>,
    pub levels: [String; 4],
    pub errata: Option<Errata>
}

impl Localization for ConditionLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct Condition {
    pub id: u32,
    pub src: SourceRefs,
    pub translations: Translations<ConditionLocalization>
}

impl Identifiable for Condition {
    fn id(&self) -> Id {
        Id::new(Category::Conditions,  self.id)
    }
}

impl Translatable for Condition {
    type Localization = ConditionLocalization;

    fn translations(&self) -> &Translations<ConditionLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
pub struct StateLocalization {
    pub name: String,
    pub description: String,
    pub errata: Option<Errata>
}

impl Localization for StateLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct State {
    pub id: u32,

    /// Is the state a prerequisite for certain entries or rules? If true, it
    /// needs to be managed by the player so they can use whatever entries or
    /// rules depend on it.
    #[serde(rename = "isPrerequisite")]
    pub is_prerequisite: bool,
    pub src: SourceRefs,
    pub translations: Translations<StateLocalization>
}

impl Identifiable for State {
    fn id(&self) -> Id {
        Id::new(Category::States, self.id)
    }
}

impl Translatable for State {
    type Localization = StateLocalization;

    fn translations(&self) -> &Translations<StateLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
pub struct Disease {
    pub id: u32,

    /// The disease's level.
    pub level: u32,
    pub src: SourceRefs,
    pub translations: ErrataTranslations
}

impl Identifiable for Disease {
    fn id(&self) -> Id {
        Id::new(Category::Diseases, self.id)
    }
}

impl Translatable for Disease {
    type Localization = ErrataLocalization;

    fn translations(&self) -> &Translations<ErrataLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum PoisonLevel {
    QL,
    Fixed(u32)
}

#[derive(Deserialize, Serialize)]
pub enum PoisonUseType {
    Weapon,
    Ingestion,
    Inhalation,
    Contact
}

#[derive(Deserialize, Serialize)]
pub enum PoisonType {
    AnimalVenom,
    PlantPoison,
    AlchemicalPoison,
    MineralPoison
}

#[derive(Deserialize, Serialize)]
pub struct Poison {
    pub id: u32,

    /// The poison's level.
    pub level: PoisonLevel,

    /// The poison's use type(s).
    #[serde(rename = "useType")]
    pub use_type: Vec<PoisonUseType>,

    /// The poison's type.
    #[serde(rename = "type")]
    pub poison_type: PoisonType,
    pub src: SourceRefs,
    pub translations: ErrataTranslations
}

impl Identifiable for Poison {
    fn id(&self) -> Id {
        Id::new(Category::Poisons, self.id)
    }
}

impl Translatable for Poison {
    type Localization = ErrataLocalization;

    fn translations(&self) -> &Translations<ErrataLocalization> {
        &self.translations
    }
}
