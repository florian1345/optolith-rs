use crate::data::{Localization, TranslationsTranslatable, Translations};
use crate::data::errata::Errata;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceLocalization {

    /// The name of the service.
    pub name: String,

    /// The description of the service.
    pub description: String,
    pub errata: Option<Errata>
}

impl Localization for ServiceLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Service {
    pub id: u32,
    pub src: SourceRefs,
    pub translations: Translations<ServiceLocalization>
}

impl TranslationsTranslatable for Service {
    type Localization = ServiceLocalization;

    fn translations(&self) -> &Translations<ServiceLocalization> {
        &self.translations
    }
}

impl Identifiable for Service {
    fn id(&self) -> Id {
        Id::new(Category::Services, self.id)
    }
}
