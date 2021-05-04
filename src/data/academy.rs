use crate::data::{Translatable, Translations};
use crate::data::effects_localization::EffectsLocalization;
use crate::data::prerequisite::InfluenceListPrerequisite;
use crate::data::simple::{SimpleLocalization, SimpleTranslations};
use crate::data::skill::non_profane::magical::StandardSpellworkId;
use crate::data::src::SourceRefs;
use crate::id::{Category, Id, Identifiable};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum ElectiveSpellworkRestriction {
    Element(u32)
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElectiveSpellwork {
    #[serde(flatten)]
    pub spellwork: StandardSpellworkId,

    /// The elective spellwork may only take effect if a certain condition is
    /// met. The condition may be related to professions or profession
    /// variants, but it is designed so that it can work without a specific
    /// profession, as multiple may belong to an institute, but with
    /// referencing other entities instead.
    pub restriction: Option<ElectiveSpellworkRestriction>
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum RestrictedSpellwork {

    /// Spellworks of a property are disallowed. It might be specified that
    /// specific spellworks are excluded from that rule. Also, spellworks of a
    /// property up to a certain number may be allowed.
    Property {
        id: u32,
        exclude: Option<Vec<StandardSpellworkId>>
    },
    Spell(u32),
    Ritual(u32),
    DemonSummoning,
    Borbaradian,
    DamageIntelligent
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpellworkAdjustment {
    pub id: StandardSpellworkId,

    /// `value` will be added to the current SR.
    pub value: u32
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpellworkChange {
    pub replacement: SpellworkAdjustment,
    pub base: SpellworkAdjustment
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(deny_unknown_fields)]
pub enum LessonPackageSkillId {
    MeleeCombatTechnique(u32),
    RangedCombatTechnique(u32),
    Skill(u32),
    Spell(u32),
    Ritual(u32)
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LessonPackageSkill {
    pub id: LessonPackageSkillId,

    /// This value will be added to the current SR/CtR.
    pub value: i32
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LessonPackage {
    pub id: u32,

    /// The spell values difference of the lesson package. This field reflects
    /// the changes (difference) to the field of the same name in the
    /// profession package. If a spell gets to SR 0 because of this, it will be
    /// removed completely.
    #[serde(rename = "spellworkChanges")]
    pub spellwork_changes: Option<Vec<SpellworkChange>>,
    pub skills: Option<Vec<LessonPackageSkill>>,
    pub translations: SimpleTranslations
}

impl Translatable for LessonPackage {
    type Localization = SimpleLocalization;

    fn translations(&self) -> &SimpleTranslations {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Curriculum {
    pub id: u32,
    pub guideline: u32,

    /// The academy's elective spellworks package.
    #[serde(rename = "electiveSpellworks")]
    pub elective_spellworks: Option<Vec<ElectiveSpellwork>>,

    /// The academy's restricted spellworks package.
    #[serde(rename = "restrictedSpellworks")]
    pub restricted_spellworks: Option<Vec<RestrictedSpellwork>>,

    /// A list of available lesson packages.
    #[serde(rename = "lessonPackages")]
    pub lesson_packages: Vec<LessonPackage>,
    pub translations: SimpleTranslations
}

impl Identifiable for Curriculum {
    fn id(&self) -> Id {
        Id::new(Category::Curricula, self.id)
    }
}

impl Translatable for Curriculum {
    type Localization = SimpleLocalization;

    fn translations(&self) -> &SimpleTranslations {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Guideline {
    pub id: u32,

    /// Maximum number of spells that can be exchanged.
    #[serde(rename = "spellworkChangesAllowed")]
    pub spellwork_changes_allowed: u32,
    pub translations: SimpleTranslations
}

impl Identifiable for Guideline {
    fn id(&self) -> Id {
        Id::new(Category::Guidelines, self.id)
    }
}

impl Translatable for Guideline {
    type Localization = SimpleLocalization;

    fn translations(&self) -> &SimpleTranslations {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Influence {
    pub id: u32,
    pub prerequisites: InfluenceListPrerequisite,
    pub src: SourceRefs,
    pub translations: Translations<EffectsLocalization>
}

impl Identifiable for Influence {
    fn id(&self) -> Id {
        Id::new(Category::Influences, self.id)
    }
}

impl Translatable for Influence {
    type Localization = EffectsLocalization;

    fn translations(&self) -> &Translations<EffectsLocalization> {
        &self.translations
    }
}
