use crate::data::{Localization, Translatable, Translations};
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AspectLocalization {
    pub name: String,
    pub master_of_aspect_suffix: Option<String>
}

impl Localization for AspectLocalization {
    fn name(&self) -> &str {
        &self.name
    }

    fn name_as_select_option(&self) -> &str {
        self.master_of_aspect_suffix.as_ref().unwrap_or(&self.name)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Aspect {
    pub id: u32,
    pub translations: Translations<AspectLocalization>
}

impl Identifiable for Aspect {
    fn id(&self) -> Id {
        Id::new(Category::Aspects, self.id)
    }
}

impl Translatable<AspectLocalization> for Aspect {
    fn translations(&self) -> &Translations<AspectLocalization> {
        &self.translations
    }
}
