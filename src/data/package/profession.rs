use crate::data::{
    Ids,
    Localization,
    SingleOrList,
    SuggestedUnsuitable,
    TranslationsTranslatable,
    Translations
};
use crate::data::activatable::SelectOptionId;
use crate::data::activatable::special_ability::SpecialAbilityId;
use crate::data::errata::Errata;
use crate::data::package::PackageSkill;
use crate::data::prerequisite::ProfessionListPrerequisite;
use crate::data::skill::combat::CombatTechniqueId;
use crate::data::skill::non_profane::karmal::KarmalWorksId;
use crate::data::skill::non_profane::magical::{
    StandardSpellworkId,
    SpellworkId
};
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum SkillSpecializationSelectOption {
    Single(Ids),
    Group(Ids)
}

/// Select an application of a skill or of one of a list of skills where you
/// get a specialization for. You can also specify one or multiple skill groups
/// from which you can choose a spell.
#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum SkillSpecializationSelectOptions {
    Single(SkillSpecializationSelectOption),
    Multiple(Vec<SkillSpecializationSelectOption>)
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectableCombatTechniques {

    /// Number of selectable CTs.
    pub number: u32,

    /// The value by which the CtRs will be increased (The base CtR is 6, to
    /// get e.g. a CtR of 8, `value` equals 2)
    pub value: u32
}

/// Select one or more combat techniques you get a CtR bonus for.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CombatTechniqueSelectOptions {

    /// Specify the number of combat techniques that can be selected so that
    /// they get increased to a specific CtR. There can be multiple selections
    /// with different CtRs.
    pub fixed: Vec<SelectableCombatTechniques>,

    /// Define if after the fixed selections the remaining unselected combat
    /// techniques' CtRs are increased by a value as well (The base CtR is 6,
    /// to get e.g. a CtR of 8, this field's value needs to equal 2).
    pub rest: Option<u32>,

    /// An array containing the combat technique ids.
    pub targets: Vec<CombatTechniqueId>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CantripSelectOptions {

    /// Number of selectable cantrips.
    pub number: u32,

    /// An array containing the cantrip ids.
    pub targets: Vec<u32>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSelectOptions {

    /// If specified, you may only choose from skills of the specified group.
    pub gr: Option<u32>,

    /// The AP value the user can spend.
    pub value: u32
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpellSelectOption {
    pub id: StandardSpellworkId,
    pub value: u32
}

/// Define one or more lists of separate select options for improving a spell
/// SR.
pub type SpellSelectOptions = Vec<Vec<SpellSelectOption>>;

/// Define one or more lists of separate select options for improving a
/// liturgical chant SR.
pub type LiturgicalChantSelectOptions = Vec<Vec<ProfessionLiturgicalChant>>;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionOptions {
    #[serde(rename = "skillSpecialization")]
    pub skill_specialization: Option<SkillSpecializationSelectOptions>,

    /// Buy languages and scripts for a specific amount of AP.
    #[serde(rename = "languageScripts")]
    pub language_scripts: Option<u32>,
    #[serde(rename = "combatTechniques")]
    pub combat_techniques: Option<CombatTechniqueSelectOptions>,
    pub cantrips: Option<CantripSelectOptions>,

    /// Buy curses for a specific amount of AP.
    pub curses: Option<u32>,

    /// Select one of a list of possible terrain knowledges. The IDs equal the
    /// selection IDs used by the special ability.
    #[serde(rename = "terrainKnowledge")]
    pub terrain_knowledge: Option<Vec<u32>>,
    pub skills: Option<SkillSelectOptions>,
    pub spells: Option<SpellSelectOptions>,
    #[serde(rename = "liturgicalChants")]
    pub liturgical_chants: Option<LiturgicalChantSelectOptions>
}

/// A special ability contained in a profession package.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionSpecialAbility {
    pub id: SpecialAbilityId,

    /// The current or required level of the entry.
    pub level: Option<u32>,

    /// Required select options. Order is important. Typically, you only need
    /// the first array index, though.
    pub options: Option<Vec<SelectOptionId>>
}

pub type ProfessionSpecialAbilities = SingleOrList<ProfessionSpecialAbility>;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionCombatTechnique {
    pub id: CombatTechniqueId,

    /// `value` will be *added* to the current CtR, which starts at 6. Example:
    /// `id: CT_1, value: 4` would result in CtR 10 for `CT_1`.
    pub value: u32
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionSpell {
    pub id: SpellworkId,

    /// `value` will be added to the current SR.
    pub value: u32
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionLiturgicalChant {
    pub id: KarmalWorksId,

    /// `value` will be added to the current SR.
    pub value: u32
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum ProfessionVariantSelectOptions<T> {
    Remove,
    Override(T)
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionVariantOptions {

    /// Either valid options or an object set to \"false\" to remove the
    /// options from the base profession.
    #[serde(rename = "skillSpecialization")]
    pub skill_specialization:
        Option<ProfessionVariantSelectOptions<
            SkillSpecializationSelectOptions>>,

    /// Either valid options or an object set to \"false\" to remove the
    /// options from the base profession.
    #[serde(rename = "languageScripts")]
    pub language_scripts: Option<ProfessionVariantSelectOptions<u32>>,
    #[serde(rename = "combatTechniques")]
    pub combat_techniques:
        Option<ProfessionVariantSelectOptions<CombatTechniqueSelectOptions>>,

    /// Either valid options or an object set to \"false\" to remove the
    /// options from the base profession.
    pub cantrips: Option<ProfessionVariantSelectOptions<CantripSelectOptions>>,

    /// Either valid options or an object set to \"false\" to remove the
    /// options from the base profession.
    pub curses: Option<ProfessionVariantSelectOptions<u32>>,

    /// Either valid options or an object set to \"false\" to remove the
    /// options from the base profession.
    #[serde(rename = "terrainKnowledge")]
    pub terrain_knowledge: Option<ProfessionVariantSelectOptions<Vec<u32>>>,

    /// Either valid options or an object set to \"false\" to remove the
    /// options from the base profession.
    pub skills: Option<ProfessionVariantSelectOptions<SkillSelectOptions>>,

    /// Either valid options or an object set to \"false\" to remove the
    /// options from the base profession.
    pub spells: Option<ProfessionVariantSelectOptions<SpellSelectOptions>>,

    /// Either valid options or an object set to \"false\" to remove the
    /// options from the base profession.
    #[serde(rename = "liturgicalChants")]
    pub liturgical_chants:
        Option<ProfessionVariantSelectOptions<LiturgicalChantSelectOptions>>
}

/// A special ability contained in or removed by a profession variant package.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionVariantSpecialAbility {
    pub id: SpecialAbilityId,

    /// If the required entry should be required to be active or inactive.
    pub active: Option<bool>,

    /// The current or required level of the entry.
    pub level: Option<u32>,

    /// Required select options. Order is important. Typically, you only need
    /// the first array index, though.
    pub options: Option<Vec<SelectOptionId>>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionVariantCombatTechnique {
    pub id: CombatTechniqueId,

    /// `value` will be *added* to the current CtR, which starts at 6. Example:
    /// `id: CT_1, value: 4` would result in CtR 10 for `CT_1`.
    pub value: i32
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionVariantSkill {
    pub id: u32,

    /// `value` will be added to the current SR.
    pub value: i32
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionVariantSpell {
    pub id: SpellworkId,

    /// `value` will be added to the current SR.
    pub value: i32
}

pub type ProfessionVariantSpells = SingleOrList<ProfessionVariantSpell>;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionVariantLiturgicalChant {
    pub id: KarmalWorksId,

    /// `value` will be added to the current SR.
    pub value: i32
}

pub type ProfessionVariantLiturgicalChants =
    SingleOrList<ProfessionVariantLiturgicalChant>;

/// If a profession name is different for male and female heroes, use this
/// object.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NameBySex {

    /// Male name.
    pub m: String,

    /// Female name.
    pub f: String
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum NameMaybeBySex {
    Universal(String),
    BySex(NameBySex)
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionVariantLocalization {
    pub name: NameMaybeBySex,

    /// Prepends this string to the generated string.
    #[serde(rename = "precedingText")]
    pub preceding_text: Option<String>,

    /// Replaces the generated string with this string.
    #[serde(rename = "fullText")]
    pub full_text: Option<String>,

    /// Appends this string to the generated string.
    #[serde(rename = "concludingText")]
    pub concluding_text: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionVariant {
    pub id: u32,

    /// The difference AP value you have to pay for the package variant. So if
    /// the profession costs 260 AP and the variant costs 266 AP, 6 has to be
    /// inserted here.
    #[serde(rename = "apValue")]
    pub ap_value: i32,
    pub prerequisites: Option<ProfessionListPrerequisite>,
    pub options: Option<ProfessionVariantOptions>,

    /// The list of special abilties contained in the profession variant. The
    /// contents of this field are merged with the field of the same name in
    /// the profession. If an entry is set to "active: false" while its
    /// "active: true" in the profession ("active: true" is default if "active"
    /// is not defined), it will be removed.
    #[serde(rename = "specialAbilities")]
    pub special_abilities: Option<Vec<ProfessionVariantSpecialAbility>>,

    /// The combat technique values difference of the profession variant. This
    /// field reflects the changes (difference) to the field of the same name
    /// in the profession package.
    #[serde(rename = "combatTechniques")]
    pub combat_techniques: Option<Vec<ProfessionVariantCombatTechnique>>,

    /// The skill values difference of the profession variant. This field
    /// reflects the changes (difference) to the field of the same name in the
    /// profession package.
    pub skills: Option<Vec<ProfessionVariantSkill>>,

    /// The spell values difference of the profession variant. This field
    /// reflects the changes (difference) to the field of the same name in the
    /// profession package. If a spell gets to SR 0 because of this, it will be
    /// removed completely.
    pub spells: Option<Vec<ProfessionVariantSpells>>,

    /// The chant values difference of the profession variant. This field
    /// reflects the changes (difference) to the field of the same name in the
    /// profession package. If a chant gets to SR 0 because of this, it will be
    /// removed completely.
    #[serde(rename = "liturgicalChants")]
    pub liturgical_chants: Option<Vec<ProfessionVariantLiturgicalChants>>,

    /// The list of blessings to activate for the profession package.
    pub blessings: Option<Vec<u32>>,
    pub translations: Translations<ProfessionVariantLocalization>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionLocalization {
    pub name: NameMaybeBySex,

    /// A name addition of the profession. This will contain texts like name of the academy or the witch circle.
    pub subname: Option<NameMaybeBySex>,

    /// Prepends the provided string to the main prerequisites string.
    #[serde(rename = "prerequisitesStart")]
    pub prerequisites_start: Option<String>,

    /// A text describing suggested advantages.
    #[serde(rename = "suggestedAdvantages")]
    pub suggested_advantages: Option<String>,

    /// A text describing suggested disadvantages.
    #[serde(rename = "suggestedDisadvantages")]
    pub suggested_disadvantages: Option<String>,

    /// A text describing unsuitable advantages.
    #[serde(rename = "unsuitableAdvantages")]
    pub unsuitable_advantages: Option<String>,

    /// The respective unsuitable disadvantages text from the source book.
    #[serde(rename = "unsuitableDisadvantages")]
    pub unsuitable_disadvantages: Option<String>,
    pub errata: Option<Errata>
}

impl Localization for ProfessionLocalization {
    fn name(&self) -> &str {
        // TODO make gender-neutral

        match &self.name {
            NameMaybeBySex::Universal(n) => n,
            NameMaybeBySex::BySex(nbs) => &nbs.m
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubProfession {
    pub id: u32,

    /// The AP value you have to pay for the package.
    #[serde(rename = "apValue")]
    pub ap_value: Option<u32>,
    pub prerequisites: Option<ProfessionListPrerequisite>,
    pub options: Option<ProfessionOptions>,

    /// The list of special abilties contained in the profession package.
    #[serde(rename = "specialAbilities")]
    pub special_abilities: Option<Vec<ProfessionSpecialAbilities>>,

    /// The combat technique values of the profession package. If you buy the
    /// profession package, the `value` will be *added* to the current CtR,
    /// which starts at 6.
    #[serde(rename = "combatTechniques")]
    pub combat_techniques: Option<Vec<ProfessionCombatTechnique>>,

    /// The skill values of the profession package. If you buy the profession
    /// package, the value will be *added* to the current SR.
    pub skills: Option<Vec<PackageSkill>>,

    /// The spell values of the profession package. If you buy the profession
    /// package, the value will be *added* to the current SR and the spell will
    /// get activated.
    pub spells: Option<Vec<ProfessionSpell>>,

    /// The chant values of the profession package. If you buy the profession
    /// package, the value will be *added* to the current SR and the chant will
    /// get activated.
    #[serde(rename = "liturgicalChants")]
    pub liturgical_chants: Option<Vec<ProfessionLiturgicalChant>>,

    /// The list of blessings to activate for the profession package.
    pub blessings: Option<Vec<u32>>,

    /// A list of suggested advantages.
    #[serde(rename = "suggestedAdvantages")]
    pub suggested_advantages: Option<Vec<SuggestedUnsuitable>>,

    /// A list of suggested disadvantages.
    #[serde(rename = "suggestedDisadvantages")]
    pub suggested_disadvantages: Option<Vec<SuggestedUnsuitable>>,

    /// A list of unsuitable advantages.
    #[serde(rename = "unsuitableAdvantages")]
    pub unsuitable_advantages: Option<Vec<SuggestedUnsuitable>>,

    /// A list of unsuitable disadvantages.
    #[serde(rename = "unsuitableDisadvantages")]
    pub unsuitable_disadvantages: Option<Vec<SuggestedUnsuitable>>,

    /// A list of available profession variants.
    pub variants: Option<Vec<ProfessionVariant>>,

    /// Whether a variant has to be selected by the user (`true`) or selecting
    /// a variant is optional (`false`).
    #[serde(rename = "isVariantRequired")]
    pub is_variant_required: bool,

    /// The curriculum's id.
    pub curriculum: Option<u32>,

    /// The profession group.
    pub gr: u32,

    /// A subgroup of `gr`.
    pub sgr: u32,
    pub src: SourceRefs,
    pub translations: Translations<ProfessionLocalization>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Profession {
    pub id: u32,

    /// A list of professions representing the same profession but with
    /// (slightly) different stats. For example, there may be a profession in a
    /// regional sourcebook or in the core rules and a profession in an
    /// extension rulebook like Magic of Aventuria, where the profession is
    /// basically called the same and almost has the same values, but the
    /// version from Magic of Aventuria features a spell style special ability
    /// that does not exist in the core rules or regional sourcebook.
    pub instances: Vec<SubProfession>
}

impl Identifiable for Profession {
    fn id(&self) -> Id {
        Id::new(Category::Professions, self.id)
    }
}

impl TranslationsTranslatable for Profession {
    type Localization = ProfessionLocalization;

    fn translations(&self) -> &Translations<ProfessionLocalization> {
        &self.instances[0].translations
    }
}
