use crate::data::{Localization, Translatable, Translations};
use crate::data::activatable::{APValue, SelectOptions};
use crate::data::errata::Errata;
use crate::data::prerequisite::GeneralListOrByLevelPrerequisite;
use crate::data::src::SourceRefs;
use crate::id::{Category, CategoryProvider, Id, Identifiable};

use serde::{Deserialize, Serialize};

use std::marker::PhantomData;

#[derive(Deserialize, Serialize)]
pub struct SimpleEnchantmentLocalization {
    
    /// The name of the entry.
    pub name: String,

    /// The name of the entry shown in Wiki. Only use when `name` needs to be
    /// different from full name.
    #[serde(rename = "nameInWiki")]
    pub name_in_wiki: Option<String>,

    /// A string that is used as a placeholder text for an input field.
    pub input: Option<String>,

    /// The effect description. Markdown is available.
    pub effect: String,

    /// The AE Cost.
    #[serde(rename = "aeCost")]
    pub ae_cost: Option<String>,

    /// The volume points the enchantment needs.
    pub volume: String,

    /// The binding cost for an enchantment.
    #[serde(rename = "bindingCost")]
    pub binding_cost: Option<String>,

    /// Use if text cannot be generated by the app. Markdown is available.
    pub prerequisites: Option<String>,

    /// Prepends the provided string to the main prerequisites string. No
    /// effect if `prerequisites` field is used in l10n file. Markdown is
    /// available.
    #[serde(rename = "prerequisitesStart")]
    pub prerequisites_start: Option<String>,

    /// Appends the provided string to the main prerequisites string. No effect
    /// if `prerequisites` field is used in l10n table. Markdown is available.
    #[serde(rename = "prerequisitesEnd")]
    pub prerequisites_end: Option<String>,

    /// The AP value. Only use this if the text provides different information
    /// than `X adventure points`, e.g. for Special Ability Property Knowledge
    /// it is \"10 adventure points for the first *Property Knowledge*, 20
    /// adventure points for the second, 40 adventure points for the third\".
    /// Markdown is available.
    #[serde(rename = "apValue")]
    pub ap_value: Option<String>,

    /// An addition to the default AP value schema. Only use this if the text
    /// provides information appended to `X adventure points` and if `apValue`
    /// is not used. Markdown is available.
    #[serde(rename = "apValueAppend")]
    pub ap_value_append: Option<String>,
    pub errata: Option<Errata>
}

impl Localization for SimpleEnchantmentLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct UnboundEnchantmentLocalization {
    
    /// The name of the entry.
    pub name: String,

    /// The name of the entry shown in Wiki. Only use when `name` needs to be
    /// different from full name.
    #[serde(rename = "nameInWiki")]
    pub name_in_wiki: Option<String>,

    /// A string that is used as a placeholder text for an input field.
    pub input: Option<String>,

    /// The effect description. Markdown is available.
    pub effect: String,

    /// The AE Cost.
    #[serde(rename = "aeCost")]
    pub ae_cost: Option<String>,

    /// Use if text cannot be generated by the app. Markdown is available.
    pub prerequisites: Option<String>,

    /// Prepends the provided string to the main prerequisites string. No
    /// effect if `prerequisites` field is used in l10n file. Markdown is
    /// available.
    #[serde(rename = "prerequisitesStart")]
    pub prerequisites_start: Option<String>,

    /// Appends the provided string to the main prerequisites string. No effect
    /// if `prerequisites` field is used in l10n table. Markdown is available.
    #[serde(rename = "prerequisitesEnd")]
    pub prerequisites_end: Option<String>,

    /// The AP value. Only use this if the text provides different information
    /// than `X adventure points`, e.g. for Special Ability Property Knowledge
    /// it is \"10 adventure points for the first *Property Knowledge*, 20
    /// adventure points for the second, 40 adventure points for the third\".
    /// Markdown is available.
    #[serde(rename = "apValue")]
    pub ap_value: Option<String>,

    /// An addition to the default AP value schema. Only use this if the text
    /// provides information appended to `X adventure points` and if `apValue`
    /// is not used. Markdown is available.
    #[serde(rename = "apValueAppend")]
    pub ap_value_append: Option<String>,
    pub errata: Option<Errata>
}

impl Localization for UnboundEnchantmentLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

/// The magic property. See `Properties` to get the id. Use DependingOnProperty
/// if there is no clear property.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum Property {
    DependingOnProperty,
    Single(Option<u32>)
}

#[derive(Deserialize, Serialize)]
pub struct BasicEnchantment<C: CategoryProvider, L: Localization> {
    pub id: u32,
    pub levels: Option<u32>,
    pub max: Option<u32>,
    #[serde(rename = "selectOptions")]
    pub select_options: Option<SelectOptions>,
    pub property: Property,
    #[serde(rename = "apValue")]
    pub ap_value: Option<APValue>,
    pub prerequisites: Option<GeneralListOrByLevelPrerequisite>,
    pub src: SourceRefs,
    pub translations: Translations<L>,
    #[serde(skip)]
    category: PhantomData<C>
}

impl<C, L> Identifiable for BasicEnchantment<C, L>
where
    C: CategoryProvider,
    L: Localization
{
    fn id(&self) -> Id {
        Id::new(C::CATEGORY, self.id)
    }
}

impl<C, L> Translatable for BasicEnchantment<C, L>
where
    C: CategoryProvider,
    L: Localization
{
    type Localization = L;

    fn translations(&self) -> &Translations<L> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
pub struct CauldronEnchantment {
    pub id: u32,
    pub levels: Option<u32>,
    pub max: Option<u32>,
    #[serde(rename = "selectOptions")]
    pub select_options: Option<SelectOptions>,
    pub property: Property,

    /// Witches can learn to brew special things in their Witch's Cauldron.
    /// These brews can be categorized in different types.
    pub brew: u32,
    #[serde(rename = "apValue")]
    pub ap_value: Option<APValue>,
    pub prerequisites: Option<GeneralListOrByLevelPrerequisite>,
    pub src: SourceRefs,
    pub translations: Translations<SimpleEnchantmentLocalization>,
}

impl Identifiable for CauldronEnchantment {
    fn id(&self) -> Id {
        Id::new(Category::CauldronEnchantments, self.id)
    }
}

impl Translatable for CauldronEnchantment {
    type Localization = SimpleEnchantmentLocalization;

    fn translations(&self) -> &Translations<SimpleEnchantmentLocalization> {
        &self.translations
    }
}

pub type SimpleEnchantment<C> =
    BasicEnchantment<C, SimpleEnchantmentLocalization>;

pub struct ArcaneOrbEnchantmentCategory;

impl CategoryProvider for ArcaneOrbEnchantmentCategory {
    const CATEGORY: Category = Category::ArcaneOrbEnchantments;
}

pub type ArcaneOrbEnchantment =
    SimpleEnchantment<ArcaneOrbEnchantmentCategory>;

pub struct AttireEnchantmentCategory;

impl CategoryProvider for AttireEnchantmentCategory {
    const CATEGORY: Category = Category::AttireEnchantments;
}

pub type AttireEnchantment = SimpleEnchantment<AttireEnchantmentCategory>;

pub struct BowlEnchantmentCategory;

impl CategoryProvider for BowlEnchantmentCategory {
    const CATEGORY: Category = Category::BowlEnchantments;
}

pub type BowlEnchantment = SimpleEnchantment<BowlEnchantmentCategory>;

pub struct ChronicleEnchantmentCategory;

impl CategoryProvider for ChronicleEnchantmentCategory {
    const CATEGORY: Category = Category::ChronicleEnchantments;
}

pub type ChronicleEnchantment =
    SimpleEnchantment<ChronicleEnchantmentCategory>;

pub struct DaggerRitualCategory;

impl CategoryProvider for DaggerRitualCategory {
    const CATEGORY: Category = Category::DaggerRituals;
}

pub type DaggerRitual = SimpleEnchantment<DaggerRitualCategory>;

pub struct FoolsHatEnchantmentCategory;

impl CategoryProvider for FoolsHatEnchantmentCategory {
    const CATEGORY: Category = Category::FoolsHatEnchantments;
}

pub type FoolsHatEnchantment = SimpleEnchantment<FoolsHatEnchantmentCategory>;

pub struct InstrumentEnchantmentCategory;

impl CategoryProvider for InstrumentEnchantmentCategory {
    const CATEGORY: Category = Category::InstrumentEnchantments;
}

pub type InstrumentEnchantment =
    SimpleEnchantment<InstrumentEnchantmentCategory>;

pub struct KrallenkettenzauberCategory;

impl CategoryProvider for KrallenkettenzauberCategory {
    const CATEGORY: Category = Category::Krallenkettenzauber;
}

pub type Krallenkettenzauber = SimpleEnchantment<KrallenkettenzauberCategory>;

pub struct OrbEnchantmentCategory;

impl CategoryProvider for OrbEnchantmentCategory {
    const CATEGORY: Category = Category::OrbEnchantments;
}

pub type OrbEnchantment = SimpleEnchantment<OrbEnchantmentCategory>;

pub struct RingEnchantmentCategory;

impl CategoryProvider for RingEnchantmentCategory {
    const CATEGORY: Category = Category::RingEnchantments;
}

pub type RingEnchantment = SimpleEnchantment<RingEnchantmentCategory>;

pub struct SickleRitualCategory;

impl CategoryProvider for SickleRitualCategory {
    const CATEGORY: Category = Category::SickleRituals;
}

pub type SickleRitual = SimpleEnchantment<SickleRitualCategory>;

pub struct SpellSwordEnchantmentCategory;

impl CategoryProvider for SpellSwordEnchantmentCategory {
    const CATEGORY: Category = Category::SpellSwordEnchantments;
}

pub type SpellSwordEnchantment =
    SimpleEnchantment<SpellSwordEnchantmentCategory>;

pub struct StaffEnchantmentCategory;

impl CategoryProvider for StaffEnchantmentCategory {
    const CATEGORY: Category = Category::StaffEnchantments;
}

pub type StaffEnchantment = SimpleEnchantment<StaffEnchantmentCategory>;

pub struct ToyEnchantmentCategory;

impl CategoryProvider for ToyEnchantmentCategory {
    const CATEGORY: Category = Category::ToyEnchantments;
}

pub type ToyEnchantment = SimpleEnchantment<ToyEnchantmentCategory>;

pub struct TrinkhornzauberCategory;

impl CategoryProvider for TrinkhornzauberCategory {
    const CATEGORY: Category = Category::Trinkhornzauber;
}

pub type Trinkhornzauber = SimpleEnchantment<TrinkhornzauberCategory>;

pub struct WandEnchantmentCategory;

impl CategoryProvider for WandEnchantmentCategory {
    const CATEGORY: Category = Category::WandEnchantments;
}

pub type WandEnchantment =
    BasicEnchantment<WandEnchantmentCategory, UnboundEnchantmentLocalization>;

pub struct WeaponEnchantmentCategory;

impl CategoryProvider for WeaponEnchantmentCategory {
    const CATEGORY: Category = Category::WeaponEnchantments;
}

pub type WeaponEnchantment = SimpleEnchantment<WeaponEnchantmentCategory>;
