use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::data::{
    Localization,
    SuggestedUnsuitable,
    Translatable,
    Translations
};
use crate::data::errata::Errata;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

#[derive(Deserialize, Serialize)]
pub struct AttributeAdjustment {
    pub id: u32,
    pub value: i32
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum DieSides {
    Three = 3,
    Six = 6,
    Twenty = 20
}

#[derive(Deserialize, Serialize)]
pub struct Die {
    pub amount: i32,
    pub sides: DieSides
}

#[derive(Deserialize, Serialize)]
pub struct StartingAge {
    #[serde(rename = "experienceLevelId")]
    pub experience_level_id: u32,
    pub base: u32,
    pub random: Die
}

#[derive(Deserialize, Serialize)]
pub struct RaceVariantLocalization {
    pub name: String,
    #[serde(rename = "commonAdvantages")]
    pub common_advantages: Option<String>,
    #[serde(rename = "commonDisadvantages")]
    pub common_disadvantages: Option<String>,
    #[serde(rename = "uncommonAdvantages")]
    pub uncommon_advantages: Option<String>,
    #[serde(rename = "uncommonDisadvantages")]
    pub uncommon_disadvantages: Option<String>
}

impl Localization for RaceVariantLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct Phenotype {
    #[serde(rename = "commonCultures")]
    pub common_cultures: Vec<u32>,
    #[serde(rename = "hairColors")]
    pub hair_colors: Vec<u32>,
    #[serde(rename = "eyeColors")]
    pub eye_colors: Vec<u32>,
    #[serde(rename = "sizeBase")]
    pub size_base: u32,
    #[serde(rename = "sizeRandom")]
    pub size_random: Vec<Die>
}

#[derive(Deserialize, Serialize)]
pub struct RaceVariant {
    pub id: u32,
    #[serde(rename = "commonAdvantages")]
    pub common_advantages: Option<Vec<SuggestedUnsuitable>>,
    #[serde(rename = "commonDisadvantages")]
    pub common_disadvantages: Option<Vec<SuggestedUnsuitable>>,
    #[serde(rename = "uncommonAdvantages")]
    pub uncommon_advantages: Option<Vec<SuggestedUnsuitable>>,
    #[serde(rename = "uncommonDisadvantages")]
    pub uncommon_disadvantages: Option<Vec<SuggestedUnsuitable>>,
    pub translations: Translations<RaceVariantLocalization>,
    #[serde(flatten)]
    pub phenotype: Phenotype
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum TypeSpecificData {
    WithVariants {
        variants: Vec<RaceVariant>
    },
    WithoutVariants(Phenotype)
}

#[derive(Deserialize, Serialize)]
pub struct RaceLocalization {
    pub name: String,
    #[serde(rename = "attributeAdjustments")]
    pub attribute_adjustments: String,
    #[serde(rename = "automaticAdvantages")]
    pub automatic_advantages: Option<String>,
    #[serde(rename = "stronglyRecommendedAdvantages")]
    pub strongly_recommended_advantages: Option<String>,
    #[serde(rename = "stronglyRecommendedDisadvantages")]
    pub strongly_recommended_disadvantages: Option<String>,
    #[serde(rename = "commonAdvantages")]
    pub common_advantages: Option<String>,
    #[serde(rename = "commonDisadvantages")]
    pub common_disadvantages: Option<String>,
    #[serde(rename = "uncommonAdvantages")]
    pub uncommon_advantages: Option<String>,
    #[serde(rename = "uncommonDisadvantages")]
    pub uncommon_disadvantages: Option<String>,
    pub errata: Option<Errata>
}

impl Localization for RaceLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct Race {
    pub id: u32,
    #[serde(rename = "apValue")]
    pub ap_value: i32,
    pub lp: i32,
    pub spi: i32,
    pub tou: i32,
    pub mov: i32,
    #[serde(rename = "attributeAdjustments")]
    pub attribute_adjustments: Option<Vec<AttributeAdjustment>>,
    #[serde(rename = "attributeAdjustmentsSelectionValue")]
    pub attribute_adjustments_selection_value: i32,
    #[serde(rename = "attributeAdjustmentsSelectionList")]
    pub attribute_adjustments_selection_list: Vec<u32>,
    #[serde(rename = "automaticAdvantages")]
    pub automatic_advantages: Option<Vec<u32>>,
    #[serde(rename = "stronglyRecommendedAdvantages")]
    pub strongly_recommended_advantages: Option<Vec<SuggestedUnsuitable>>,
    #[serde(rename = "stronglyRecommendedDisadvantages")]
    pub strongly_recommended_disadvantages: Option<Vec<SuggestedUnsuitable>>,
    #[serde(rename = "commonAdvantages")]
    pub common_advantages: Option<Vec<SuggestedUnsuitable>>,
    #[serde(rename = "commonDisadvantages")]
    pub common_disadvantages: Option<Vec<SuggestedUnsuitable>>,
    #[serde(rename = "uncommonAdvantages")]
    pub uncommon_advantages: Option<Vec<SuggestedUnsuitable>>,
    #[serde(rename = "uncommonDisadvantages")]
    pub uncommon_disadvantages: Option<Vec<SuggestedUnsuitable>>,
    #[serde(rename = "weightBase")]
    pub weight_base: u32,
    #[serde(rename = "weightRandom")]
    pub weight_random: Vec<Die>,
    #[serde(rename = "startingAge")]
    pub starting_age: Vec<StartingAge>,
    #[serde(rename = "typeSpecific")]
    pub type_specific: TypeSpecificData,
    pub src: SourceRefs,
    pub translations: Translations<RaceLocalization>
}

impl Identifiable for Race {
    fn id(&self) -> Id {
        Id::new(Category::Races, self.id)
    }
}

impl Translatable for Race {
    type Localization = RaceLocalization;

    fn translations(&self) -> &Translations<RaceLocalization> {
        &self.translations
    }
}
