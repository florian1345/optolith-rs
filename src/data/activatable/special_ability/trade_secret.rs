use crate::data::{Localization, Translatable, Translations};
use crate::data::errata::Errata;
use crate::data::prerequisite::ProfessionListPrerequisite;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TradeSecretLocalization {
    pub name: String,

    /// The description of the trade secret. Markdown is available.
    pub description: Option<String>,
    pub errata: Option<Errata>
}

impl Localization for TradeSecretLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TradeSecret {
    pub id: u32,

    /// AP value of the trade secret.
    #[serde(rename = "apValue")]
    pub ap_value: u32,

    /// Is this trade secret considered secret knowledge?
    #[serde(rename = "isSecretKnowledge")]
    pub is_secret_knowledge: bool,
    pub prerequisites: Option<ProfessionListPrerequisite>,
    pub src: SourceRefs,
    pub translations: Translations<TradeSecretLocalization>
}

impl Identifiable for TradeSecret {
    fn id(&self) -> Id {
        Id::new(Category::TradeSecrets, self.id)
    }
}

impl Translatable for TradeSecret {
    type Localization = TradeSecretLocalization;

    fn translations(&self) -> &Translations<TradeSecretLocalization> {
        &self.translations
    }
}
