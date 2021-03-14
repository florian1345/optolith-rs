use serde::{Deserialize, Serialize};

use crate::data::{Localization, Translatable, Translations};
use crate::id::{Category, Id, Identifiable};

#[derive(Deserialize, Serialize)]
pub struct DerivedCharacteristicLocalization {
    pub name: String,
    #[serde(rename = "nameAbbr")]
    pub name_abbr: String,
    pub calc: String,
    #[serde(rename = "calcHalfPrimary")]
    pub calc_half_primary: Option<String>,
    #[serde(rename = "calcNoPrimary")]
    pub calc_no_primary: Option<String>
}


impl Localization for DerivedCharacteristicLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct DerivedCharacteristic {
    pub id: u32,
    pub translations: Translations<DerivedCharacteristicLocalization>
}

impl Identifiable for DerivedCharacteristic {
    fn id(&self) -> Id {
        Id::new(Category::DerivedCharacteristics, self.id)
    }
}

impl Translatable for DerivedCharacteristic {
    type Localization = DerivedCharacteristicLocalization;

    fn translations(&self)
            -> &Translations<DerivedCharacteristicLocalization> {
        &self.translations
    }
}
