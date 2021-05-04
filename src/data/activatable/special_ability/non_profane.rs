use crate::data::{Localization, Translatable, Translations};
use crate::data::activatable::special_ability::{
    AdvancedSpecialAbilities,
    EffectSpecialAbilityLocalization,
    SimpleSpecialAbility,
    SkillInfluencingSpecialAbility,
    SpecialAbilityLocalization
};
use crate::data::activatable::special_ability::ordinary::{
    OrdinarySpecialAbility,
    OrdinarySkillInfluencingSpecialAbility
};
use crate::data::errata::Errata;
use crate::id::{Category, CategoryProvider, Id, Identifiable};

use serde::{Deserialize, Serialize};

pub struct MagicalSpecialAbilityCategory;

impl CategoryProvider for MagicalSpecialAbilityCategory {
    const CATEGORY: Category = Category::MagicalSpecialAbilities;
}

pub type MagicalSpecialAbility =
    OrdinarySkillInfluencingSpecialAbility<MagicalSpecialAbilityCategory>;

pub struct KarmaSpecialAbilityCategory;

impl CategoryProvider for KarmaSpecialAbilityCategory {
    const CATEGORY: Category = Category::KarmaSpecialAbilities;
}

pub type KarmaSpecialAbility =
    OrdinarySpecialAbility<KarmaSpecialAbilityCategory>;

pub struct AdvancedMagicalSpecialAbilityCategory;

impl CategoryProvider for AdvancedMagicalSpecialAbilityCategory {
    const CATEGORY: Category = Category::AdvancedMagicalSpecialAbilities;
}

pub type AdvancedMagicalSpecialAbility =
    OrdinarySpecialAbility<AdvancedMagicalSpecialAbilityCategory>;

pub struct AdvancedKarmaSpecialAbilityCategory;

impl CategoryProvider for AdvancedKarmaSpecialAbilityCategory {
    const CATEGORY: Category = Category::AdvancedKarmaSpecialAbilities;
}

pub type AdvancedKarmaSpecialAbility =
    OrdinarySkillInfluencingSpecialAbility<AdvancedKarmaSpecialAbilityCategory>;

pub struct SermonCategory;
    
impl CategoryProvider for SermonCategory {
    const CATEGORY: Category = Category::Sermons;
}
    
pub type Sermon = OrdinarySpecialAbility<SermonCategory>;

pub struct VisionCategory;

impl CategoryProvider for VisionCategory {
    const CATEGORY: Category = Category::Visions;
}

pub type Vision = OrdinarySpecialAbility<VisionCategory>;

pub struct FamiliarSpecialAbilityCategory;

impl CategoryProvider for FamiliarSpecialAbilityCategory {
    const CATEGORY: Category = Category::FamiliarSpecialAbilities;
}

pub type FamiliarSpecialAbility =
    SimpleSpecialAbility<FamiliarSpecialAbilityCategory,
        EffectSpecialAbilityLocalization>;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AncestorGlyphLocalization {

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

    #[serde(rename = "aeCost")]
    pub ae_cost: String,

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

impl Localization for AncestorGlyphLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct AncestorGlyphCategory;

impl CategoryProvider for AncestorGlyphCategory {
    const CATEGORY: Category = Category::AncestorGlyphs;
}

pub type AncestorGlyph = SimpleSpecialAbility<AncestorGlyphCategory,
    AncestorGlyphLocalization>;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectiveWardingCircleSpecialAbilityLocalization {
    pub name: String,

    /// The name of the entry shown in Wiki. Only use when `name` needs to be
    /// different from full name.
    #[serde(rename = "nameInWiki")]
    pub name_in_wiki: Option<String>,
    #[serde(rename = "aeCost")]
    pub ae_cost: String,

    /// The rules for the protective circle variant. Markdown is available.
    #[serde(rename = "protectiveCircle")]
    pub protective_circle: String,

    /// The rules for the warding circle variant. Markdown is available.
    #[serde(rename = "wardingCircle")]
    pub warding_circle: String,

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

impl Localization for ProtectiveWardingCircleSpecialAbilityLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct ProtectiveWardingCircleSpecialAbilityCategory;

impl CategoryProvider for ProtectiveWardingCircleSpecialAbilityCategory {
    const CATEGORY: Category =
        Category::ProtectiveWardingCircleSpecialAbilities;
}

pub type ProtectiveWardingCircleSpecialAbility =
    SimpleSpecialAbility<ProtectiveWardingCircleSpecialAbilityCategory,
        ProtectiveWardingCircleSpecialAbilityLocalization>;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NonProfaneStyleSpecialAbility<C: CategoryProvider> {
    #[serde(flatten)]
    pub data: SkillInfluencingSpecialAbility<C, SpecialAbilityLocalization>,
    pub advanced: AdvancedSpecialAbilities
}

impl<C: CategoryProvider> Identifiable for NonProfaneStyleSpecialAbility<C> {
    fn id(&self) -> Id {
        self.data.id()
    }
}

impl<C: CategoryProvider> Translatable for NonProfaneStyleSpecialAbility<C> {
    type Localization = SpecialAbilityLocalization;

    fn translations(&self) -> &Translations<SpecialAbilityLocalization> {
        self.data.translations()
    }
}

// TODO figure out why C suddenly requires Deserialize, Serialize

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MagicStyleSpecialAbilityCategory;

impl CategoryProvider for MagicStyleSpecialAbilityCategory {
    const CATEGORY: Category = Category::MagicStyleSpecialAbilities;
}

pub type MagicStyleSpecialAbility =
    NonProfaneStyleSpecialAbility<MagicStyleSpecialAbilityCategory>;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LiturgicalStyleSpecialAbilityCategory;

impl CategoryProvider for LiturgicalStyleSpecialAbilityCategory {
    const CATEGORY: Category = Category::LiturgicalStyleSpecialAbilities;
}

pub type LiturgicalStyleSpecialAbility =
    NonProfaneStyleSpecialAbility<LiturgicalStyleSpecialAbilityCategory>;
