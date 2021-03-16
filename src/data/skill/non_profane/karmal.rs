use crate::data::{Translatable, Translations};
use crate::data::skill::non_profane::{
    CheckMod,
    Enhancements,
    NonProfaneSkillLocalization,
    SmallNonProfaneSkillLocalization,
    Traditions
};
use crate::data::skill::ImprovementCost;
use crate::data::src::SourceRefs;
use crate::id::{Category, CategoryProvider, Id, Identifiable};

use serde::{Deserialize, Serialize};

use std::marker::PhantomData;

#[derive(Deserialize, Serialize)]
pub struct ImprovableKarmalSkill<C: CategoryProvider> {
    pub id: u32,
    pub check: [u32; 3],

    /// If the check will be modified by Spirit or Toughness, insert `SPI` or
    /// `TOU` respectively.
    #[serde(rename = "checkMod")]
    pub check_mod: Option<CheckMod>,
    pub ic: ImprovementCost,

    /// The tradition(s) the chant/ceremony is available for. The integers
    /// represent the tradition ids.
    pub traditions: Traditions,

    /// The aspect(s) of the tradition(s) the chant/ceremony is part of. The
    /// integers represent the aspect IDs.
    pub aspects: Option<Vec<u32>>,

    /// Is the casting time not modifiable?
    #[serde(rename = "castingTimeNoMod")]
    pub casting_time_no_mod: bool,

    /// Is the KP cost not modifiable?
    #[serde(rename = "costNoMod")]
    pub cost_no_mod: bool,

    /// Is the range not modifiable?
    #[serde(rename = "rangeNoMod")]
    pub range_no_mod: bool,

    /// Is the duration not modifiable?
    #[serde(rename = "durationNoMod")]
    pub duration_no_mod: bool,
    pub enhancements: Option<Enhancements>,
    pub src: SourceRefs,
    pub translations: Translations<NonProfaneSkillLocalization>,
    #[serde(skip)]
    category: PhantomData<C>
}

impl<C: CategoryProvider> Identifiable for ImprovableKarmalSkill<C> {
    fn id(&self) -> Id {
        Id::new(C::CATEGORY, self.id)
    }
}

impl<C> Translatable for ImprovableKarmalSkill<C>
where
    C: CategoryProvider
{
    type Localization = NonProfaneSkillLocalization;

    fn translations(&self) -> &Translations<NonProfaneSkillLocalization> {
        &self.translations
    }
}

pub struct LiturgicalChantCategory;

impl CategoryProvider for LiturgicalChantCategory {
    const CATEGORY: Category = Category::LiturgicalChants;
}

pub type LiturgicalChant = ImprovableKarmalSkill<LiturgicalChantCategory>;

pub struct CeremonyCategory;

impl CategoryProvider for CeremonyCategory {
    const CATEGORY: Category = Category::Ceremonies;
}

pub type Ceremony = ImprovableKarmalSkill<CeremonyCategory>;

#[derive(Deserialize, Serialize)]
pub struct Blessing {
    pub id: u32,
    pub src: SourceRefs,
    pub translations: Translations<SmallNonProfaneSkillLocalization>
}

impl Identifiable for Blessing {
    fn id(&self) -> Id {
        Id::new(Category::Blessings, self.id)
    }
}

impl Translatable for Blessing {
    type Localization = SmallNonProfaneSkillLocalization;

    fn translations(&self) -> &Translations<SmallNonProfaneSkillLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum KarmalWorksId {
    LiturgicalChant(u32),
    Ceremony(u32)
}
