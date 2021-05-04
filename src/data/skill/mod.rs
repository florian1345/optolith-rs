use serde::{Deserialize, Serialize};

use crate::data::{Localization, Translatable, Translations};
use crate::data::errata::Errata;
use crate::data::simple::SimpleTranslations;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

pub mod combat;
pub mod non_profane;

/// This is one data structure used for "applications" and "uses".
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExplicitApplication {
    pub id: i32,
    pub translations: SimpleTranslations
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum DerivedApplications {
    Regions,
    Diseases
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum Applications {
    Derived(DerivedApplications),
    Explicit(Vec<ExplicitApplication>)
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ImprovementCost {
    A,
    B,
    C,
    D
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum EncumbranceAffected {
    #[serde(rename = "true")]
    Yes,
    #[serde(rename = "false")]
    No,
    #[serde(rename = "maybe")]
    Maybe
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillLocalization {
    pub name: String,
    #[serde(rename = "applicationsInput")]
    pub applications_input: Option<String>,
    #[serde(rename = "encDescription")]
    pub enc_description: Option<String>,
    pub tools: Option<String>,
    pub quality: String,
    pub failed: String,
    pub critical: String,
    pub botch: String,
    pub errata: Option<Errata>
}

impl Localization for SkillLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Skill {
    pub id: u32,
    pub applications: Option<Applications>,
    pub check: [u32; 3],
    pub ic: ImprovementCost,
    pub enc: EncumbranceAffected,
    pub gr: u32,
    pub src: SourceRefs,
    pub translations: Translations<SkillLocalization>
}

impl Identifiable for Skill {
    fn id(&self) -> Id {
        Id::new(Category::Skills,  self.id)
    }
}

impl Translatable for Skill {
    type Localization = SkillLocalization;

    fn translations(&self) -> &Translations<SkillLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillGroupLocalization {
    pub name: String,
    #[serde(rename = "fullName")]
    pub full_name: String
}

impl Localization for SkillGroupLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillGroup {
    pub id: u32,
    pub check: [u32; 3],
    pub translations: Translations<SkillGroupLocalization>
}

impl Identifiable for SkillGroup {
    fn id(&self) -> Id {
        Id::new(Category::SkillGroups, self.id)
    }
}

impl Translatable for SkillGroup {
    type Localization = SkillGroupLocalization;

    fn translations(&self) -> &Translations<SkillGroupLocalization> {
        &self.translations
    }
}
