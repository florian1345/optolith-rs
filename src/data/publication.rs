use crate::data::{Localization, Translatable, Translations};
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum PublicationType {
    CoreRules,
    ExpansionRules,
    Sourcebook,
    RegionalSourcebook,
    Adventure
}

#[derive(Deserialize, Serialize)]
pub struct PrintingLocalization {

    /// The printing's number.
    pub number: u32
}

#[derive(Deserialize, Serialize)]
pub struct Printing {

    /// The universal printing identifier.
    pub id: u32,
    pub translations: Translations<PrintingLocalization>
}

#[derive(Deserialize, Serialize)]
pub struct PublicationLocalization {

    /// The book's publisher ID.
    pub id: Option<String>,

    /// The name of the book.
    pub name: String,

    /// The abbreviation of the book's name
    #[serde(rename = "nameAbbr")]
    pub name_abbr: String,

    /// The book's release date.
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,

    /// The book is not (fully) implemented and thus needs to be excluded from
    /// the stable releases.
    #[serde(rename = "isMissingImplementation")]
    pub is_missing_implementation: Option<bool>
}

impl Localization for PublicationLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct Publication {
    pub id: u32,

    /// Does the book contain explicit/adult content?
    #[serde(rename = "isAdultContent")]
    pub is_adult_content: bool,

    /// The publication type.
    #[serde(rename = "type")]
    pub pub_type: PublicationType,

    /// The book is not (fully) implemented and thus needs to be excluded from
    /// the stable releases.
    #[serde(rename = "isMissingImplementation")]
    pub is_missing_implementation: Option<bool>,

    /// Printings are used to provide a unified way to differenciate printings
    /// across all languages.
    pub printings: Option<Vec<Printing>>,
    pub translations: Translations<PublicationLocalization>
}

impl Identifiable for Publication {
    fn id(&self) -> Id {
        Id::new(Category::Publications, self.id)
    }
}

impl Translatable for Publication {
    type Localization = PublicationLocalization;

    fn translations(&self) -> &Translations<PublicationLocalization> {
        &self.translations
    }
}
