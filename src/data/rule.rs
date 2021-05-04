use crate::data::{Localization, Translatable, Translations};
use crate::data::errata::Errata;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuleLocalization {

    /// The name of the rule.
    pub name: String,

    /// A description of the rule.
    pub description: String,
    pub errata: Option<Errata>
}

impl Localization for RuleLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FocusRule {
    pub id: u32,

    /// The subject ID.
    pub subject: u32,

    /// The level of the focus rule.
    pub level: u32,
    pub src: SourceRefs,
    pub translations: Translations<RuleLocalization>
}

impl Identifiable for FocusRule {
    fn id(&self) -> Id {
        Id::new(Category::FocusRules, self.id)
    }
}

impl Translatable for FocusRule {
    type Localization = RuleLocalization;

    fn translations(&self) -> &Translations<RuleLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalRule {
    pub id: u32,

    /// Does the optional rule have an impact on character creation? If true,
    /// it needs to be managed by the player so they can customize character
    /// creation to their needs.
    #[serde(rename = "isPrerequisite")]
    pub is_prerequisite: bool,
    pub src: SourceRefs,
    pub translations: Translations<RuleLocalization>
}

impl Identifiable for OptionalRule {
    fn id(&self) -> Id {
        Id::new(Category::OptionalRules, self.id)
    }
}

impl Translatable for OptionalRule {
    type Localization = RuleLocalization;

    fn translations(&self) -> &Translations<RuleLocalization> {
        &self.translations
    }
}
