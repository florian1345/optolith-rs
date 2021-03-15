use crate::data::{Localization, Translatable, Translations};
use crate::data::activatable::{APValue, SelectOptions};
use crate::data::errata::Errata;
use crate::data::prerequisite::GeneralListOrByLevelPrerequisite;
use crate::data::src::SourceRefs;
use crate::id::{Category, CategoryProvider, Id, Identifiable};

use serde::{Deserialize, Serialize};

use std::marker::PhantomData;

pub mod ceremonial_item;
pub mod combat;
pub mod enchantment;
pub mod gifts;
pub mod non_profane;
pub mod skill;
pub mod tradition;

#[derive(Deserialize, Serialize)]
pub struct SpecialAbilityLocalization {

    /// The name of the entry.
    pub name: String,

    /// The name of the entry shown in Wiki. Only use when `name` needs to be
    /// different from full name.
    #[serde(rename = "nameInWiki")]
    pub name_in_wiki: Option<String>,

    /// A string that is used as a placeholder text for an input field.
    pub input: Option<String>,

    /// The rule text. Markdown is available.
    pub rules: String,

    /// The penalty the special ability gives when used.
    pub penalty: Option<String>,

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

impl Localization for SpecialAbilityLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

/// A normal special ability localization with a field `effect` instead of
/// `rules`.
#[derive(Deserialize, Serialize)]
pub struct EffectSpecialAbilityLocalization {
    
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

impl Localization for EffectSpecialAbilityLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum SingleAdvancedSpecialAbilityOption {
    Single(u32),
    Multiple(Vec<u32>)
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum AdvancedSpecialAbility {
    Single {
        id: u32,

        /// Specify, if only one specific select option or one of a set of
        /// select options is allowed for the referenced advanced special
        /// ability.
        option: Option<SingleAdvancedSpecialAbilityOption>
    },
    Multiple(Vec<u32>)
}

/// The Advanced Special Abilities for the respective Style Special Ability.
/// Sometimes, only a specific select option or a set of select options of an
/// entry is allowed, which can be modelled by the option property. It can also
/// be that you can choose from a set of special abilities, but then you cannot
/// specify an option.
pub type AdvancedSpecialAbilities = [AdvancedSpecialAbility; 3];

#[derive(Deserialize, Serialize)]
pub struct SimpleSpecialAbility<C: CategoryProvider, L: Localization> {
    pub id: u32,
    pub levels: Option<u32>,
    pub max: Option<u32>,
    #[serde(rename = "selectOptions")]
    pub select_options: Option<SelectOptions>,
    pub prerequisites: Option<GeneralListOrByLevelPrerequisite>,
    #[serde(rename = "apValue")]
    pub ap_value: Option<APValue>,
    pub src: SourceRefs,
    pub translations: Translations<L>,
    #[serde(skip)]
    category: PhantomData<C>
}

impl<C, L> Translatable for SimpleSpecialAbility<C, L>
where
    C: CategoryProvider,
    L: Localization
{
    type Localization = L;

    fn translations(&self) -> &Translations<L> {
        &self.translations
    }
}

impl<C, L> Identifiable for SimpleSpecialAbility<C, L>
where
    C: CategoryProvider,
    L: Localization
{
    fn id(&self) -> Id {
        Id::new(C::CATEGORY, self.id)
    }
}

pub struct GeneralSpecialAbilityCategory;

impl CategoryProvider for GeneralSpecialAbilityCategory {
    const CATEGORY: Category = Category::GeneralSpecialAbilities;
}

pub type GeneralSpecialAbility =
    SimpleSpecialAbility<GeneralSpecialAbilityCategory,
        SpecialAbilityLocalization>;
