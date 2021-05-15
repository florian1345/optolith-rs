use crate::data::{Localization, TranslationsTranslatable, Translations};
use crate::data::errata::Errata;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone, Deserialize, Serialize)]
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

impl TranslationsTranslatable for FocusRule {
    type Localization = RuleLocalization;

    fn translations(&self) -> &Translations<RuleLocalization> {
        &self.translations
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum OptionalRuleRelevance {
    Extraneous,
    Linked {

        /// Does the optional rule have an impact on character creation or
        /// character sheet and this effect has not been implemented in
        /// Optolith yet? If \"true\", the optional rule cannot be activated.
        #[serde(default)]
        #[serde(rename = "isMissingImplementation")]
        is_missing_implementation: bool
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalRule {
    pub id: u32,

    /// The relevance of the optional rule for Optolith. It may be that it
    /// influences character creating but it may also just influnce the
    /// character sheet. If it is linked to systems in Optolith, it may be
    /// specified if this rule has not been implemented in Optolith yet.
    pub relevance: OptionalRuleRelevance,

    /// Does the optional rule have an impact on character creation or
    /// character sheet and this effect has not been implemented in Optolith
    /// yet? If \"true\", the optional rule cannot be activated.
    #[serde(default)]
    #[serde(rename = "isMissingImplementation")]
    pub is_missing_implementation: bool,
    pub src: SourceRefs,
    pub translations: Translations<RuleLocalization>
}

impl Identifiable for OptionalRule {
    fn id(&self) -> Id {
        Id::new(Category::OptionalRules, self.id)
    }
}

impl TranslationsTranslatable for OptionalRule {
    type Localization = RuleLocalization;

    fn translations(&self) -> &Translations<RuleLocalization> {
        &self.translations
    }
}
