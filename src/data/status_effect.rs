use crate::data::{Localization, Translations, TranslationsTranslatable};
use crate::data::errata::Errata;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

impl TranslationsTranslatable for Condition {
    type Localization = ConditionLocalization;

    fn translations(&self) -> &Translations<ConditionLocalization> {
        &self.translations
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

impl TranslationsTranslatable for State {
    type Localization = StateLocalization;

    fn translations(&self) -> &Translations<StateLocalization> {
        &self.translations
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum Resistance {
    Spirit,
    Toughness
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiseaseCauseLocalization {

    /// The cause's name.
    pub name: String,

    /// The chance to get infected by this cause. If present for this language,
    /// this overrides the universal `chance` property; they cannot be used at
    /// the same time.
    pub chance: Option<String>
}

/// A single disease cause.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiseaseCause {

    /// The change to get infected by this cause, in percent.
    pub chance: Option<u32>,
    pub translations: Translations<DiseaseCauseLocalization>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum AlternativeName {
    General(String),
    InRegion {
        name: String,

        /// The region where this alternative name is used.
        region: String
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiseaseProgressDependent {

    /// The information for the disease's default progress.
    pub default: String,

    /// The information for the disease's lessened progress.
    pub lessened: Option<String>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiseaseLocalization {
    pub name: String,

    /// A list of alternative names.
    #[serde(rename = "alternativeNames")]
    pub alternative_names: Option<Vec<AlternativeName>>,

    /// The disease's progress, in detail.
    // TODO remove Option once possible
    pub progress: Option<String>,

    /// After infection, how much time passes before symptoms appear?
    // TODO remove Option once possible
    #[serde(rename = "incubationTime")]
    pub incubation_time: Option<String>,

    /// The damage caused by the disease. If the disease check fails, apply the
    /// lessened effects.
    // TODO remove Option once possible
    pub damage: Option<DiseaseProgressDependent>,

    /// The duration of the disease. If the disease check fails, use the
    /// lessened duration.
    // TODO remove Option once possible
    pub duration: Option<DiseaseProgressDependent>,

    /// Special information about the disease.
    pub special: Option<String>,

    /// Methods known to lessen the disease’s progress or relieve symptoms.
    /// Markdown is available.
    // TODO remove Option once possible
    pub treatment: Option<String>,

    /// Known remedies for the disease.
    // TODO remove Option once possible
    pub cure: Option<String>,
    pub errata: Option<Errata>
}

impl Localization for DiseaseLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Disease {
    pub id: u32,

    /// The disease's level.
    pub level: u32,

    /// Depending on the disease, apply Spirit or Toughness as a penalty to the
    /// disease roll.
    // TODO remove Option whenever possible
    pub resistance: Option<Resistance>,

    /// What causes the disease? The GM rolls 1D20 to see if a character gets
    /// infected. If the infection check succeeds, the GM makes a disease check
    /// to determine the severity of the infection.
    // TODO remove Option whenever possible
    pub cause: Option<Vec<DiseaseCause>>,
    pub src: SourceRefs,
    pub translations: Translations<DiseaseLocalization>
}

impl Identifiable for Disease {
    fn id(&self) -> Id {
        Id::new(Category::Diseases, self.id)
    }
}

impl TranslationsTranslatable for Disease {
    type Localization = DiseaseLocalization;

    fn translations(&self) -> &Translations<DiseaseLocalization> {
        &self.translations
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommunicationCauseLocalization {

    /// The cause's name.
    pub name: String,

    /// The chance to get infected by this cause. If present for this language,
    /// this overrides the universal `chance` property; they cannot be used at
    /// the same time.
    pub chance: Option<String>,

    /// An additional note about this communication cause.
    pub node: Option<String>
}

/// A single cause.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommunicationCause {

    /// The change to get infected by this cause, in percent.
    pub chance: Option<u32>,
    pub translations: Translations<CommunicationCauseLocalization>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum Communicability {
    NonCommunicable,
    Communicable(Vec<CommunicationCause>)
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnimalDisease {
    pub id: u32,

    /// The disease's level.
    pub level: u32,

    /// Depending on the disease, apply Spirit or Toughness as a penalty to the
    /// disease roll.
    pub resistance: Resistance,

    /// What causes the disease? The GM rolls 1D20 to see if a character gets
    /// infected. If the infection check succeeds, the GM makes a disease check
    /// to determine the severity of the infection.
    pub cause: Vec<DiseaseCause>,

    /// The animal types this disease applies to. If empty, it applies to all
    /// animal types.
    #[serde(rename = "animalTypes")]
    pub animal_types: Vec<u32>,

    /// If the animal disease is communicable to intelligent creatures and the
    /// circumstances of a communication.
    #[serde(rename = "communicabilityToIntelligentCreatures")]
    pub communicability_to_intelligent_creatures: Communicability,
    pub src: SourceRefs,
    pub translations: Translations<DiseaseLocalization>
}

impl Identifiable for AnimalDisease {
    fn id(&self) -> Id {
        Id::new(Category::AnimalDiseases, self.id)
    }
}

impl TranslationsTranslatable for AnimalDisease {
    type Localization = DiseaseLocalization;

    fn translations(&self) -> &Translations<DiseaseLocalization> {
        &self.translations
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum PoisonLevel {
    QL,
    Fixed(u32)
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum PoisonApplicationType {
    Weapon,
    Ingestion,
    Inhalation,
    Contact
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum Legality {
    Legal,
    Illegal
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntoxicantLocalization {

    /// How to ingest the intoxicant.
    pub ingestion: String,

    /// The intoxicant's side effects, if any. Markdown is available.
    #[serde(rename = "sideEffects")]
    pub side_effects: Option<String>,

    /// What happens if the intoxicant has been overdosed. Markdown is
    /// available.
    pub overdose: String,
    pub special: Option<String>,
    pub addiction: Option<String>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "category")]
#[serde(deny_unknown_fields)]
pub enum PlantPoisonData {
    Default,
    Intoxicant {

        /// Whether the use of the intoxicant is legal or not.
        legality: Legality,
        translations: Translations<IntoxicantLocalization>
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "sourceType", content = "categorySpecific")]
#[serde(deny_unknown_fields)]
pub enum PoisonSourceType {
    AnimalVenom,
    PlantPoison(Option<PlantPoisonData>),
    AlchemicalPoison,
    MineralPoison
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PoisonProgressDependent {

    /// The information if the poison gets the default effect.
    pub default: String,

    /// The information if the poison gets the degraded effect.
    pub degraded: String
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PoisonLocalization {

    /// The poison's name.
    pub name: String,

    /// A list of alternative names.
    #[serde(rename = "alternativeNames")]
    pub alternative_names: Option<Vec<AlternativeName>>,

    /// The normal and degraded poison's effects. Markdown is available.
    // TODO remove Option once possible
    pub effect: Option<PoisonProgressDependent>,

    /// When the poison takes effect.
    // TODO remove Option once possible
    pub start: Option<String>,

    /// The normal and degraded poison's duration.
    // TODO remove Option once possible
    pub duration: Option<PoisonProgressDependent>,
    pub errata: Option<Errata>
}

impl Localization for PoisonLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Poison {
    pub id: u32,

    /// The poison's level.
    pub level: PoisonLevel,

    /// The poison's application type(s).
    #[serde(rename = "applicationType")]
    pub application_type: Vec<PoisonApplicationType>,

    /// The poison's source type and dependent additional values.
    #[serde(rename = "sourceTypeSpecific")]
    pub source_type_specific: PoisonSourceType,

    /// Depending on the poison, apply Spirit or Toughness as a penalty to the
    /// poison roll.
    // TODO remove Option once possible
    pub resistance: Option<Resistance>,

    /// The raw (ingredients) value, in silverthalers.
    // TODO remove Option once possible
    pub value: Option<u32>,

    /// Price for one dose, in silverthalers.
    // TODO remove Option once possible
    pub cost: Option<u32>,
    pub src: SourceRefs,
    pub translations: Translations<PoisonLocalization>
}

impl Identifiable for Poison {
    fn id(&self) -> Id {
        Id::new(Category::Poisons, self.id)
    }
}

impl TranslationsTranslatable for Poison {
    type Localization = PoisonLocalization;

    fn translations(&self) -> &Translations<PoisonLocalization> {
        &self.translations
    }
}
