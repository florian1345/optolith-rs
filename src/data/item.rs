use crate::data::TranslationsTranslatable;
use crate::data::errata::{ErrataLocalization, ErrataTranslations};
use crate::data::simple::SimpleEntity;
use crate::data::src::SourceRefs;
use crate::id::{Category, CategoryProvider, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemStack {
    pub id: u32,
    pub amount: Option<u32>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EquipmentPackage {
    pub id: u32,

    /// All items in the package. You have to provide the item (template) ID
    /// and you can optionally provide the amount how often an item is included
    /// in the package.
    pub items: Vec<ItemStack>,
    pub src: SourceRefs,
    pub translations: ErrataTranslations
}

impl Identifiable for EquipmentPackage {
    fn id(&self) -> Id {
        Id::new(Category::EquipmentPackages, self.id)
    }
}

impl TranslationsTranslatable for EquipmentPackage {
    type Localization = ErrataLocalization;

    fn translations(&self) -> &ErrataTranslations {
        &self.translations
    }
}

#[derive(Clone)]
pub struct ItemGroupCategory;

impl CategoryProvider for ItemGroupCategory {
    const CATEGORY: Category = Category::ItemGroups;
}

pub type ItemGroup = SimpleEntity<ItemGroupCategory>;
