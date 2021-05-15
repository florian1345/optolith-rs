use crate::data::{
    Localization, 
    SuggestedUnsuitable,
    TranslationsTranslatable,
    Translations
};
use crate::data::errata::Errata;
use crate::data::package::PackageSkill;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum ProfessionsException {
    Single(u32),
    Group(u32)
}

pub type ProfessionsExceptions = Vec<ProfessionsException>;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CultureLocalization {

    /// The name of the culture.
    pub name: String,

    /// The full description of possible area knowledges.
    #[serde(rename = "areaKnowledge")]
    pub area_knowledge: String,

    /// The type of area knowledge.
    #[serde(rename = "areaKnowledgeShort")]
    pub area_knowledge_short: String,

    /// The text describing common mundane professions.
    #[serde(rename = "commonMundaneProfessions")]
    pub common_mundane_professions: Option<String>,

    /// The text describing common magical professions.
    #[serde(rename = "commonMagicalProfessions")]
    pub common_magical_professions: Option<String>,

    /// The text describing common blessed professions.
    #[serde(rename = "commonBlessedProfessions")]
    pub common_blessed_professions: Option<String>,

    /// A text describing common advantages.
    #[serde(rename = "commonAdvantages")]
    pub common_advantages: Option<String>,

    /// A text describing common disadvantages.
    #[serde(rename = "commonDisadvantages")]
    pub common_disadvantages: Option<String>,

    /// A text describing uncommon advantages.
    #[serde(rename = "uncommonAdvantages")]
    pub uncommon_advantages: Option<String>,

    /// A text describing uncommon disadvantages.
    #[serde(rename = "uncommonDisadvantages")]
    pub uncommon_disadvantages: Option<String>,

    /// A text describing common names.
    #[serde(rename = "commonNames")]
    pub common_names: String,
    pub errata: Option<Errata>
}

impl Localization for CultureLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Culture {
    pub id: u32,

    /// A list of native languages (usually it is only one).
    pub languages: Vec<u32>,

    /// A list of native literacy (usually it is only one). If the culture does
    /// not use any script, leave this field empty.
    pub literacy: Option<Vec<u32>>,

    /// A list of possible social status in the respective culture.
    pub social: Vec<u32>,

    /// If all mundane professions are common for this culture or not.
    /// Exceptions can be specified in `commonMundaneProfessionsExceptions`.
    #[serde(rename = "commonMundaneProfessionsAll")]
    pub common_mundane_professions_all: bool,

    /// If `commonMundaneProfessionsAll` is `true`, listed professions (integer
    /// id) or profession groups are excluded from common professions list. If
    /// `commonMundaneProfessionsAll` is `false` (or empty), listed professions
    /// (integer id) or profession groups are the only common mundane
    /// professions.
    #[serde(rename = "commonMundaneProfessionsExceptions")]
    pub common_mundane_professions_exceptions: Option<ProfessionsExceptions>,

    /// If all magical professions are common for this culture or not.
    /// Exceptions can be specified in `commonMagicalProfessionsExceptions`.
    #[serde(rename = "commonMagicalProfessionsAll")]
    pub common_magical_professions_all: bool,

    /// If `commonMagicalProfessionsAll` is `true`, listed professions (integer
    /// id) or profession groups are excluded from common professions list. If
    /// `commonMagicalProfessionsAll` is `false` (or empty), listed professions
    /// (integer id) or profession groups are the only common magical
    /// professions.
    #[serde(rename = "commonMagicalProfessionsExceptions")]
    pub common_magical_professions_exceptions: Option<ProfessionsExceptions>,

    /// If all blessed professions are common for this culture or not.
    /// Exceptions can be specified in `commonBlessedProfessionsExceptions`.
    #[serde(rename = "commonBlessedProfessionsAll")]
    pub common_blessed_professions_all: bool,

    /// If `commonBlessedProfessionsAll` is `true`, listed professions (integer
    /// id) or profession groups are excluded from common professions list. If
    /// `commonBlessedProfessionsAll` is `false` (or empty), listed professions
    /// (integer id) or profession groups are the only common blessed
    /// professions.
    #[serde(rename = "commonBlessedProfessionsExceptions")]
    pub common_blessed_professions_exceptions: Option<ProfessionsExceptions>,

    /// A list of common advantages.
    #[serde(rename = "commonAdvantages")]
    pub common_advantages: Option<Vec<SuggestedUnsuitable>>,

    /// A list of common disadvantages.
    #[serde(rename = "commonDisadvantages")]
    pub common_disadvantages: Option<Vec<SuggestedUnsuitable>>,

    /// A list of uncommon advantages.
    #[serde(rename = "uncommonAdvantages")]
    pub uncommon_advantages: Option<Vec<SuggestedUnsuitable>>,

    /// A list of uncommon disadvantages.
    #[serde(rename = "uncommonDisadvantages")]
    pub uncommon_disadvantages: Option<Vec<SuggestedUnsuitable>>,

    /// A list of common skills.
    #[serde(rename = "commonSkills")]
    pub common_skills: Vec<u32>,

    /// A list of uncommon skills.
    #[serde(rename = "uncommonSkills")]
    pub uncommon_skills: Option<Vec<u32>>,

    /// The skill points you get for buying the culture package.
    #[serde(rename = "culturalPackageSkills")]
    pub cultural_package_skills: Vec<PackageSkill>,
    pub src: SourceRefs,
    pub translations: Translations<CultureLocalization>
}

impl Identifiable for Culture {
    fn id(&self) -> Id {
        Id::new(Category::Cultures, self.id)
    }
}

impl TranslationsTranslatable for Culture {
    type Localization = CultureLocalization;

    fn translations(&self) -> &Translations<CultureLocalization> {
        &self.translations
    }
}
