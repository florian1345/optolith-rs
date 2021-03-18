use crate::data::{Ids, Localization, Translatable, Translations};
use crate::data::errata::Errata;
use crate::data::skill::non_profane::{
    CheckMod,
    Enhancements,
    MainParameterLocalization,
    NonProfaneSkillLocalization,
    QualityLevelEffectLocalization,
    SmallNonProfaneSkillLocalization
};
use crate::data::prerequisite::{
    ActivatableListPrerequisite,
    IncreasableListPrerequisite
};
use crate::data::simple::SimpleTranslations;
use crate::data::skill::ImprovementCost;
use crate::data::src::SourceRefs;
use crate::id::{Category, CategoryProvider, Id, Identifiable};

use serde::{Deserialize, Serialize};

use std::marker::PhantomData;

/// A spell or ritual.
#[derive(Deserialize, Serialize)]
pub struct MagicalSkill<C: CategoryProvider> {
    pub id: u32,
    pub check: [u32; 3],
    
    #[serde(rename = "checkMod")]
    pub check_mod: Option<CheckMod>,
    pub ic: ImprovementCost,
    pub traditions: Vec<u32>,

    /// The tradition(s) the spell is available for, but where the traditions
    /// does not exist as an SA yet. The integers represent the tradition
    /// placeholder identifiers.
    #[serde(rename = "traditionPlaceholders")]
    pub tradition_placeholders: Option<Vec<u32>>,

    /// The property ID.
    pub property: u32,

    /// Is the casting time not modifiable?
    #[serde(rename = "castingTimeNoMod")]
    pub casting_time_no_mod: bool,

    /// Is the AE cost not modifiable?
    #[serde(rename = "costNoMod")]
    pub cost_no_mod: bool,

    /// Is the range not modifiable?
    #[serde(rename = "rangeNoMod")]
    pub range_no_mod: bool,

    /// Is the duration not modifiable?
    #[serde(rename = "durationNoMod")]
    pub duration_no_mod: bool,

    /// A list of Increasable prerequisites that have to be met.
    pub prerequisites: Option<IncreasableListPrerequisite>,
    pub enhancements: Option<Enhancements>,
    pub src: SourceRefs,
    pub translations: Translations<NonProfaneSkillLocalization>,
    #[serde(skip)]
    category: PhantomData<C>
}

impl<C: CategoryProvider> Identifiable for MagicalSkill<C> {
    fn id(&self) -> Id {
        Id::new(C::CATEGORY, self.id)
    }
}

impl<C> Translatable for MagicalSkill<C>
where
    C: CategoryProvider
{
    type Localization = NonProfaneSkillLocalization;

    fn translations(&self) -> &Translations<NonProfaneSkillLocalization> {
        &self.translations
    }
}

pub struct SpellCategory;

impl CategoryProvider for SpellCategory {
    const CATEGORY: Category = Category::Spells;
}

pub type Spell = MagicalSkill<SpellCategory>;

pub struct RitualCategory;

impl CategoryProvider for RitualCategory {
    const CATEGORY: Category = Category::Rituals;
}

pub type Ritual = MagicalSkill<RitualCategory>;

#[derive(Deserialize, Serialize)]
pub struct Cantrip {
    pub id: u32,

    /// The tradition(s) the cantrip is available for. The integers represent
    /// the tradition ids.
    pub traditions: Vec<u32>,

    /// The property ID.
    pub property: u32,

    /// A list of Activatable prerequisites that have to be met.
    pub prerequisites: Option<ActivatableListPrerequisite>,
    pub src: SourceRefs,
    pub translations: Translations<SmallNonProfaneSkillLocalization>
}

impl Identifiable for Cantrip {
    fn id(&self) -> Id {
        Id::new(Category::Cantrips, self.id)
    }
}

impl Translatable for Cantrip {
    type Localization = SmallNonProfaneSkillLocalization;

    fn translations(&self) -> &Translations<SmallNonProfaneSkillLocalization> {
        &self.translations
    }
}

/// Stores the information about a music tradition in the context of a magical
/// dance/melody.
#[derive(Deserialize, Serialize)]
pub struct MusicTraditionSpecificData {

    /// The music tradition's id.
    pub id: u32,

    /// Translations of the music-tradition specific name of this dance/melody.
    pub translations: SimpleTranslations
}

/// A localization of a magical dance or melody.
#[derive(Deserialize, Serialize)]
pub struct MusicalMagicLocalization {

    /// The name of the dance/melody.
    pub name: String,

    /// The effect description. Markdown is available. If the effect is
    /// different for different quality levels, use `effectQualityLevels`. If
    /// there is general effect text after the list of quality levels, use
    /// `effectAfterQualityLevels` for that.
    pub effect: String,
    #[serde(rename = "effectQualityLevels")]
    pub effect_quality_levels: Option<QualityLevelEffectLocalization>,

    /// The effect description after the quality levels list. Markdown is
    /// available.
    #[serde(rename = "effectAfterQualityLevels")]
    pub effect_after_quality_levels: Option<String>,

    /// The duration.
    pub duration: MainParameterLocalization,

    /// The AE cost.
    pub cost: MainParameterLocalization,
    pub errata: Option<Errata>
}

impl Localization for MusicalMagicLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct MagicalDance {
    pub id: u32,
    pub check: [u32; 3],

    /// The property ID.
    pub property: u32,

    /// The music tradition(s) the magical dance is available for. This also
    /// defines the different names in each music tradition.
    #[serde(rename = "musicTradition")]
    pub music_tradition: Vec<MusicTraditionSpecificData>,
    pub ic: ImprovementCost,
    pub src: SourceRefs,
    pub translations: Translations<MusicalMagicLocalization>
}

impl Identifiable for MagicalDance {
    fn id(&self) -> Id {
        Id::new(Category::MagicalDances, self.id)
    }
}

impl Translatable for MagicalDance {
    type Localization = MusicalMagicLocalization;

    fn translations(&self) -> &Translations<MusicalMagicLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
pub struct MagicalMelody {
    pub id: u32,
    pub check: [u32; 3],

    /// The skill(s) that can be used for enhancing the melody.
    pub skill: Ids,

    /// The property ID.
    pub property: u32,

    /// The music tradition(s) the magical melody is available for. This also
    /// defines the different names in each music tradition.
    #[serde(rename = "musicTradition")]
    pub music_tradition: Vec<MusicTraditionSpecificData>,
    pub ic: ImprovementCost,
    pub src: SourceRefs,
    pub translations: Translations<MusicalMagicLocalization>
}

impl Identifiable for MagicalMelody {
    fn id(&self) -> Id {
        Id::new(Category::MagicalMelodies, self.id)
    }
}

impl Translatable for MagicalMelody {
    type Localization = MusicalMagicLocalization;

    fn translations(&self) -> &Translations<MusicalMagicLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
pub struct ElvenMagicalSongLocalization {

    /// The name of the elven magical song.
    pub name: String,

    /// The effect description. Markdown is available.
    pub effect: String,

    /// The AE cost.
    pub cost: MainParameterLocalization,
    pub errata: Option<Errata>
}

impl Localization for ElvenMagicalSongLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct ElvenMagicalSong {
    pub id: u32,
    pub check: [u32; 3],

    /// If the check will be modified by Spirit or Toughness, insert `SPI` or
    /// `TOU` respectively. If the higher is the characteristic to choose,
    /// insert an array with both instead.
    #[serde(rename = "checkMod")]
    pub check_mod: Option<CheckMod>,

    /// The skill(s) that can be used for enhancing the song.
    pub skill: Ids,

    /// The property ID.
    pub property: u32,
    pub ic: ImprovementCost,
    pub src: SourceRefs,
    pub translations: Translations<ElvenMagicalSongLocalization>
}

impl Identifiable for ElvenMagicalSong {
    fn id(&self) -> Id {
        Id::new(Category::ElvenMagicalSongs, self.id)
    }
}

impl Translatable for ElvenMagicalSong {
    type Localization = ElvenMagicalSongLocalization;

    fn translations(&self) -> &Translations<ElvenMagicalSongLocalization> {
        &self.translations
    }
}

/// A struct which contains localizations for spells or rituals which have no
/// specified range or casting time.
#[derive(Deserialize, Serialize)]
pub struct NoRangeTimeLocalization {
    pub name: String,
    pub effect: String,
    pub cost: MainParameterLocalization,
    pub duration: MainParameterLocalization,
    pub errata: Option<Errata>
}

impl Localization for NoRangeTimeLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

/// A struct for rituals for one specific tradition that share many rules such
/// that they do not appear in the schema.
#[derive(Deserialize, Serialize)]
pub struct SimpleMagicalSkill<C: CategoryProvider, L: Localization> {
    pub id: u32,
    pub check: [u32; 3],
    
    /// If the check will be modified by Spirit or Toughness, insert `SPI` or
    /// `TOU` respectively. If the higher is the characteristic to choose,
    /// insert an array with both instead.
    #[serde(rename = "checkMod")]
    pub check_mod: Option<CheckMod>,

    /// The property ID.
    pub property: u32,

    /// A list of Activatable prerequisites that have to be met.
    pub prerequisites: Option<ActivatableListPrerequisite>,
    pub src: SourceRefs,
    pub translations: Translations<L>,
    #[serde(skip)]
    category: PhantomData<C>
}

impl<C, L> Identifiable for SimpleMagicalSkill<C, L>
where
    C: CategoryProvider,
    L: Localization
{
    fn id(&self) -> Id {
        Id::new(C::CATEGORY, self.id)
    }
}

impl<C, L> Translatable for SimpleMagicalSkill<C, L>
where
    C: CategoryProvider,
    L: Localization
{
    type Localization = L;

    fn translations(&self) -> &Translations<L> {
        &self.translations
    }
}

pub struct CurseCategory;

impl CategoryProvider for CurseCategory {
    const CATEGORY: Category = Category::Curses;
}

pub type Curse = SimpleMagicalSkill<CurseCategory, NoRangeTimeLocalization>;

pub struct DominationRitualCategory;

impl CategoryProvider for DominationRitualCategory {
    const CATEGORY: Category = Category::DominationRituals;
}

pub type DominationRitual =
    SimpleMagicalSkill<DominationRitualCategory, NoRangeTimeLocalization>;

pub struct GeodeRitualCategory;

impl CategoryProvider for GeodeRitualCategory {
    const CATEGORY: Category = Category::GeodeRituals;
}

pub type GeodeRitual =
    SimpleMagicalSkill<GeodeRitualCategory, NonProfaneSkillLocalization>;

#[derive(Deserialize, Serialize)]
pub struct ZibiljaRitual {
    pub id: u32,
    pub check: [u32; 3],
    
    /// If the check will be modified by Spirit or Toughness, insert `SPI` or
    /// `TOU` respectively. If the higher is the characteristic to choose,
    /// insert an array with both instead.
    #[serde(rename = "checkMod")]
    pub check_mod: Option<CheckMod>,
    pub ic: ImprovementCost,

    /// The property ID.
    pub property: u32,

    /// Is the casting time not modifiable?
    #[serde(rename = "castingTimeNoMod")]
    pub casting_time_no_mod: bool,

    /// Is the AE cost not modifiable?
    #[serde(rename = "costNoMod")]
    pub cost_no_mod: bool,

    /// Is the range not modifiable?
    #[serde(rename = "rangeNoMod")]
    pub range_no_mod: bool,

    /// Is the duration not modifiable?
    #[serde(rename = "durationNoMod")]
    pub duration_no_mod: bool,
    pub src: SourceRefs,
    pub translations: Translations<NonProfaneSkillLocalization>
}

impl Identifiable for ZibiljaRitual {
    fn id(&self) -> Id {
        Id::new(Category::ZibiljaRituals, self.id)
    }
}

impl Translatable for ZibiljaRitual {
    type Localization = NonProfaneSkillLocalization;

    fn translations(&self) -> &Translations<NonProfaneSkillLocalization> {
        &self.translations
    }
}

#[derive(Serialize, Deserialize)]
pub struct AnimistPower {
    pub id: u32,
    pub check: [u32; 3],

    /// The property ID.
    pub property: u32,

    /// The tribes the animist force is available for. Leave empty if it is
    /// available to all tribes.
    pub tribes: Option<Vec<u32>>,
    pub ic: ImprovementCost,
    pub src: SourceRefs,
    pub translations: Translations<NoRangeTimeLocalization>
}

impl Identifiable for AnimistPower {
    fn id(&self) -> Id {
        Id::new(Category::AnimistPowers, self.id)
    }
}

impl Translatable for AnimistPower {
    type Localization = NoRangeTimeLocalization;

    fn translations(&self) -> &Translations<NoRangeTimeLocalization> {
        &self.translations
    }
}

#[derive(Serialize, Deserialize)]
pub struct JesterTrick {
    pub id: u32,
    pub check: [u32; 3],

    /// If the check will be modified by Spirit or Toughness, insert `SPI` or
    /// `TOU` respectively. If the higher is the characteristic to choose,
    /// insert an array with both instead.
    #[serde(rename = "checkMod")]
    pub check_mod: Option<CheckMod>,

    /// The property ID.
    pub property: u32,
    pub ic: ImprovementCost,
    pub src: SourceRefs,
    pub translations: Translations<NonProfaneSkillLocalization>
}

impl Identifiable for JesterTrick {
    fn id(&self) -> Id {
        Id::new(Category::JesterTricks, self.id)
    }
}

impl Translatable for JesterTrick {
    type Localization = NonProfaneSkillLocalization;

    fn translations(&self) -> &Translations<NonProfaneSkillLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
pub enum MagicalRuneCheckMod {
    CombatTechnique
}

#[derive(Deserialize, Serialize)]
pub struct SpeedSeparatedMainParameterLocalization {
    pub slow: MainParameterLocalization,
    pub fast: MainParameterLocalization
}

#[derive(Deserialize, Serialize)]
pub struct MagicalRuneLocalization {

    /// The name of the magical rune.
    pub name: String,

    /// The native name of the magical rune.
    #[serde(rename = "nativeName")]
    pub native_name: String,

    /// The effect description. Markdown is available. If the effect is
    /// different for different quality levels, use "effectQualityLevels". If
    /// there is general effect text after the list of quality levels, use
    /// "effectAfterQualityLevels" for that.
    pub effect: String,
    #[serde(rename = "effectQualityLevels")]
    pub effect_quality_levels: Option<QualityLevelEffectLocalization>,

    /// The effect description after the quality levels list. Markdown is
    /// available.
    #[serde(rename = "effectAfterQualityLevels")]
    pub effect_after_quality_levels: Option<String>,

    /// The AE/KE cost.
    pub cost: MainParameterLocalization,

    /// The crafting time.
    #[serde(rename = "craftingTime")]
    pub crafting_time: SpeedSeparatedMainParameterLocalization,

    /// The duration.
    pub duration: SpeedSeparatedMainParameterLocalization,
    pub errata: Option<Errata>
}

impl Localization for MagicalRuneLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct MagicalRune {
    pub id: u32,
    pub check: [u32; 3],

    /// If the check will be modified by any value, list it here.
    #[serde(rename = "checkMod")]
    pub check_mod: Option<MagicalRuneCheckMod>,

    /// The property ID.
    pub property: u32,
    pub ic: ImprovementCost,
    pub src: SourceRefs,
    pub translations: Translations<MagicalRuneLocalization>
}

impl Identifiable for MagicalRune {
    fn id(&self) -> Id {
        Id::new(Category::MagicalRunes, self.id)
    }
}

impl Translatable for MagicalRune {
    type Localization = MagicalRuneLocalization;

    fn translations(&self) -> &Translations<MagicalRuneLocalization> {
        &self.translations
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum StandardSpellworkId {
    Spell(u32),
    Ritual(u32)
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum SpellworkId {
    Spell(u32),
    Ritual(u32),
    Curse(u32),
    ElvenMagicalSong(u32),
    DominationRitual(u32),
    MagicalMelody(u32),
    MagicalDance(u32),
    JesterTrick(u32),
    AnimistPower(u32),
    GeodeRitual(u32),
    ZibiljaRitual(u32)
}
