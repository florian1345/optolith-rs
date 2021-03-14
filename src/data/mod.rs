use crate::data::activatable::character_trait::{
    Advantage,
    Disadvantage
};
use crate::data::activatable::special_ability::{
    GeneralSpecialAbility,
    combat::{
        AdvancedCombatSpecialAbility,
        BrawlingSpecialAbility,
        CombatSpecialAbility,
        CombatStyleSpecialAbility,
        CommandSpecialAbility
    },
    non_profane::{
        AdvancedKarmaSpecialAbility,
        AdvancedMagicalSpecialAbility,
        AncestorGlyph,
        KarmaSpecialAbility,
        LiturgicalStyleSpecialAbility,
        MagicalSpecialAbility,
        MagicStyleSpecialAbility
    },
    skill::{
        AdvancedSkillSpecialAbility,
        SkillStyleSpecialAbility
    },
    tradition::{
        ArcaneBardTradition,
        ArcaneDancerTradition,
        BlessedTradition,
        MagicalTradition
    }
};
use crate::data::aspect::Aspect;
use crate::data::attribute::Attribute;
use crate::data::condition::Condition;
use crate::data::derived_characteristic::DerivedCharacteristic;
use crate::data::experience_level::ExperienceLevel;
use crate::data::language::{Language, Script};
use crate::data::non_profane_skill::karmal::{
    Blessing,
    Ceremony,
    LiturgicalChant
};
use crate::data::non_profane_skill::magical::{
    AnimistPower,
    Curse,
    DominationRitual,
    ElvenMagicalSong,
    GeodeRitual,
    JesterTrick,
    MagicalDance,
    MagicalMelody,
    Ritual,
    Spell,
    ZibiljaRitual
};
use crate::data::race::Race;
use crate::data::simple::{
    Brew,
    Element,
    EyeColor,
    HairColor,
    ItemGroup,
    LiturgicalChantGroup,
    Reach,
    SocialStatus,
    SpellGroup,
    Subject,
    Tribe
};
use crate::data::skill::{Skill, SkillGroup};
use crate::id::{Category, Id, Identifiable};
use crate::error::OptolithDataResult;
use crate::util;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::DirEntry;
use std::hash::Hash;
use std::path::PathBuf;

pub mod activatable;
pub mod aspect;
pub mod attribute;
pub mod combat_technique;
pub mod condition;
pub mod derived_characteristic;
pub mod errata;
pub mod experience_level;
pub mod language;
pub mod non_profane_skill;
pub mod prerequisite;
pub mod race;
pub mod simple;
pub mod skill;
pub mod src;

const ADVANCED_COMBAT_SPECIAL_ABILITY_DIR: &'static str =
    "AdvancedCombatSpecialAbilities";
const ADVANCED_KARMA_SPECIAL_ABILITY_DIR: &'static str =
    "AdvancedKarmaSpecialAbilities";
const ADVANCED_MAGICAL_SPECIAL_ABILITY_DIR: &'static str =
    "AdvancedMagicalSpecialAbilities";
const ADVANCED_SKILL_SPECIAL_ABILITY_DIR: &'static str =
    "AdvancedSkillSpecialAbilities";
const ADVANTAGE_DIR: &'static str = "Advantages";
const ANCESTOR_GLYPH_DIR: &'static str = "AncestorGlyphs";
const ANIMIST_POWER_DIR: &'static str = "AnimistPowers";
const ARCANE_BARD_TRADITION_DIR: &'static str = "ArcaneBardTraditions";
const ARCANE_DANCER_TRADITION_DIR: &'static str = "ArcaneDancerTraditions";
const ASPECT_DIR: &'static str = "Aspects";
const ATTRIBUTE_DIR: &'static str = "Attributes";
const BLESSED_TRADITION_DIR: &'static str = "BlessedTraditions";
const BLESSING_DIR: &'static str = "Blessings";
const BRAWLING_SPECIAL_ABILITY_DIR: &'static str = "BrawlingSpecialAbilities";
const BREW_DIR: &'static str = "Brews";
const CEREMONY_DIR: &'static str = "Ceremonies";
const COMBAT_SPECIAL_ABILITY_DIR: &'static str = "CombatSpecialAbilities";
const COMBAT_STYLE_SPECIAL_ABILITY_DIR: &'static str =
    "CombatStyleSpecialAbilities";
const COMMAND_SPECIAL_ABILITY_DIR: &'static str = "CommandSpecialAbilities";
const CONDITION_DIR: &'static str = "Conditions";
const CURSE_DIR: &'static str = "Curses";
const DERIVED_CHARACTERISTIC_DIR: &'static str = "DerivedCharacteristics";
const DISADVANTAGE_DIR: &'static str = "Disadvantages";
const DOMINATION_RITUAL_DIR: &'static str = "DominationRituals";
const ELEMENT_DIR: &'static str = "Elements";
const ELVEN_MAGICAL_SONG_DIR: &'static str = "ElvenMagicalSongs";
const EXPERIENCE_LEVEL_DIR: &'static str = "ExperienceLevels";
const EYE_COLOR_DIR: &'static str = "EyeColors";
const GENERAL_SPECIAL_ABILITY_DIR: &'static str = "GeneralSpecialAbilities";
const GEODE_RITUAL_DIR: &'static str = "GeodeRituals";
const HAIR_COLOR_DIR: &'static str = "HairColors";
const ITEM_GROUP_DIR: &'static str = "ItemGroups";
const JESTER_TRICK_DIR: &'static str = "JesterTricks";
const KARMA_SPECIAL_ABILITY_DIR: &'static str = "KarmaSpecialAbilities";
const LANGUAGE_DIR: &'static str = "Languages";
const LITURGICAL_CHANT_DIR: &'static str = "LiturgicalChants";
const LITURGICAL_CHANT_GROUP_DIR: &'static str = "LiturgicalChantGroups";
const LITURGICAL_STYLE_SPECIAL_ABILITY_DIR: &'static str =
    "LiturgicalStyleSpecialAbilities";
const MAGICAL_DANCE_DIR: &'static str = "MagicalDances";
const MAGICAL_MELODY_DIR: &'static str = "MagicalMelodies";
const MAGICAL_SPECIAL_ABILITY_DIR: &'static str = "MagicalSpecialAbilities";
const MAGICAL_TRADITION_DIR: &'static str = "MagicalTraditions";
const MAGIC_STYLE_SPECIAL_ABILITY_DIR: &'static str =
    "MagicStyleSpecialAbilities";
const RACE_DIR: &'static str = "Races";
const REACH_DIR: &'static str = "Reaches";
const RITUAL_DIR: &'static str = "Rituals";
const SCRIPT_DIR: &'static str = "Scripts";
const SKILL_DIR: &'static str = "Skills";
const SKILL_GROUP_DIR: &'static str = "SkillGroups";
const SKILL_STYLE_SPECIAL_ABILITY_DIR: &'static str =
    "SkillStyleSpecialAbilities";
const SOCIAL_STATUS_DIR: &'static str = "SocialStatuses";
const SPELL_DIR: &'static str = "Spells";
const SPELL_GROUP_DIR: &'static str = "SpellGroups";
const SUBJECT_DIR: &'static str = "Subjects";
const TRIBE_DIR: &'static str = "Tribes";
const UI_DIR: &'static str = "UI";
const ZIBILJA_RITUAL_DIR: &'static str = "ZibiljaRituals";

type UI = HashMap<String, String>;

type IdMap<T> = HashMap<u32, T>;

/// Stores the data from Optolith resource files in dictionaries to make it
/// accessible by IDs.
#[derive(Deserialize, Serialize)]
pub struct OptolithData {
    advanced_combat_special_abilities: IdMap<AdvancedCombatSpecialAbility>,
    advanced_karma_special_abilities: IdMap<AdvancedKarmaSpecialAbility>,
    advanced_magical_special_abilities: IdMap<AdvancedMagicalSpecialAbility>,
    advanced_skill_special_abilities: IdMap<AdvancedSkillSpecialAbility>,
    advantages: IdMap<Advantage>,
    ancestor_glyphs: IdMap<AncestorGlyph>,
    animist_powers: IdMap<AnimistPower>,
    arcane_bard_traditions: IdMap<ArcaneBardTradition>,
    arcane_dancer_traditions: IdMap<ArcaneDancerTradition>,
    aspects: IdMap<Aspect>,
    attributes: IdMap<Attribute>,
    blessed_traditions: IdMap<BlessedTradition>,
    blessings: IdMap<Blessing>,
    brawling_special_abilities: IdMap<BrawlingSpecialAbility>,
    brews: IdMap<Brew>,
    ceremonies: IdMap<Ceremony>,
    combat_special_abilities: IdMap<CombatSpecialAbility>,
    combat_style_special_abilities: IdMap<CombatStyleSpecialAbility>,
    command_special_abilities: IdMap<CommandSpecialAbility>,
    conditions: IdMap<Condition>,
    curses: IdMap<Curse>,
    derived_characteristics: IdMap<DerivedCharacteristic>,
    disadvantages: IdMap<Disadvantage>,
    domination_rituals: IdMap<DominationRitual>,
    elements: IdMap<Element>,
    elven_magical_songs: IdMap<ElvenMagicalSong>,
    experience_levels: IdMap<ExperienceLevel>,
    eye_colors: IdMap<EyeColor>,
    general_special_abilities: IdMap<GeneralSpecialAbility>,
    geode_rituals: IdMap<GeodeRitual>,
    hair_colors: IdMap<HairColor>,
    item_groups: IdMap<ItemGroup>,
    jester_tricks: IdMap<JesterTrick>,
    karma_special_abilities: IdMap<KarmaSpecialAbility>,
    languages: IdMap<Language>,
    liturgical_chants: IdMap<LiturgicalChant>,
    liturgical_chant_groups: IdMap<LiturgicalChantGroup>,
    liturgical_style_special_abilities: IdMap<LiturgicalStyleSpecialAbility>,
    magical_dances: IdMap<MagicalDance>,
    magical_melodies: IdMap<MagicalMelody>,
    magical_traditions: IdMap<MagicalTradition>,
    magical_special_abilities: IdMap<MagicalSpecialAbility>,
    magic_style_special_abilities: IdMap<MagicStyleSpecialAbility>,
    races: IdMap<Race>,
    reaches: IdMap<Reach>,
    rituals: IdMap<Ritual>,
    scripts: IdMap<Script>,
    skills: IdMap<Skill>,
    skill_groups: IdMap<SkillGroup>,
    skill_style_special_abilities: IdMap<SkillStyleSpecialAbility>,
    social_statuses: IdMap<SocialStatus>,
    spells: IdMap<Spell>,
    spell_groups: IdMap<SpellGroup>,
    subjects: IdMap<Subject>,
    tribes: IdMap<Tribe>,
    zibilja_rituals: IdMap<ZibiljaRitual>,
    uis: HashMap<String, UI>
}

fn is_placeholder(path: &str) -> bool {
    if let Some(underscore) = path.find('_') {
        match path[0..underscore].parse::<u32>() {
            Ok(n) => n >= 1000,
            Err(_) => true
        }
    }
    else {
        true
    }
}

fn construct_map<K, V>(dir: PathBuf, key_builder: impl Fn(&V, &DirEntry) -> K)
    -> OptolithDataResult<HashMap<K, V>>
where
    for<'de> V : Deserialize<'de>,
    K: Eq + Hash
{
    let mut map: HashMap<K, V> = HashMap::new();

    for file in util::read_dir(&dir)? {
        let file = file?;

        // TODO remove once a more permanent solution for placeholders has been
        // found

        if is_placeholder(file.file_name().to_str().unwrap()) {
            continue;
        }

        let object: V = util::deserialize_yaml_file(&file.path())?;
        let id = key_builder(&object, &file);
        map.insert(id, object);
    }

    Ok(map)
}

fn construct_u32_map<V>(dir: PathBuf) -> OptolithDataResult<IdMap<V>>
where
    for<'de> V : Deserialize<'de> + Identifiable
{
    construct_map(dir, |v: &V, _| v.id().internal_id())
}

impl OptolithData {
    pub fn from_directory(path: &str) -> OptolithDataResult<OptolithData> {
        let advanced_combat_special_abilities =
            construct_u32_map(
                util::join(path, ADVANCED_COMBAT_SPECIAL_ABILITY_DIR))?;
        let advanced_karma_special_abilities =
            construct_u32_map(
                util::join(path, ADVANCED_KARMA_SPECIAL_ABILITY_DIR))?;
        let advanced_magical_special_abilities =
            construct_u32_map(
                util::join(path, ADVANCED_MAGICAL_SPECIAL_ABILITY_DIR))?;
        let advanced_skill_special_abilities =
            construct_u32_map(
                util::join(path, ADVANCED_SKILL_SPECIAL_ABILITY_DIR))?;
        let advantages = construct_u32_map(util::join(path, ADVANTAGE_DIR))?;
        let ancestor_glyphs =
            construct_u32_map(util::join(path, ANCESTOR_GLYPH_DIR))?;
        let animist_forces =
            construct_u32_map(util::join(path, ANIMIST_POWER_DIR))?;
        let arcane_bard_traditions =
            construct_u32_map(util::join(path, ARCANE_BARD_TRADITION_DIR))?;
        let arcane_dancer_traditions =
            construct_u32_map(util::join(path, ARCANE_DANCER_TRADITION_DIR))?;
        let aspects = construct_u32_map(util::join(path, ASPECT_DIR))?;
        let attributes = construct_u32_map(util::join(path, ATTRIBUTE_DIR))?;
        let blessed_traditions =
            construct_u32_map(util::join(path, BLESSED_TRADITION_DIR))?;
        let blessings = construct_u32_map(util::join(path, BLESSING_DIR))?;
        let brawling_special_abilities =
            construct_u32_map(util::join(path, BRAWLING_SPECIAL_ABILITY_DIR))?;
        let brews = construct_u32_map(util::join(path, BREW_DIR))?;
        let ceremonies = construct_u32_map(util::join(path, CEREMONY_DIR))?;
        let combat_special_abilities =
            construct_u32_map(util::join(path, COMBAT_SPECIAL_ABILITY_DIR))?;
        let combat_style_special_abilities =
            construct_u32_map(
                util::join(path, COMBAT_STYLE_SPECIAL_ABILITY_DIR))?;
        let command_special_abilities =
            construct_u32_map(util::join(path, COMMAND_SPECIAL_ABILITY_DIR))?;
        let conditions = construct_u32_map(util::join(path, CONDITION_DIR))?;
        let curses = construct_u32_map(util::join(path, CURSE_DIR))?;
        let derived_characteristics =
            construct_u32_map(util::join(path, DERIVED_CHARACTERISTIC_DIR))?;
        let disadvantages =
            construct_u32_map(util::join(path, DISADVANTAGE_DIR))?;
        let domination_rituals =
            construct_u32_map(util::join(path, DOMINATION_RITUAL_DIR))?;
        let elements = construct_u32_map(util::join(path, ELEMENT_DIR))?;
        let elven_magical_songs =
            construct_u32_map(util::join(path, ELVEN_MAGICAL_SONG_DIR))?;
        let experience_levels =
            construct_u32_map(util::join(path, EXPERIENCE_LEVEL_DIR))?;
        let eye_colors = construct_u32_map(util::join(path, EYE_COLOR_DIR))?;
        let general_special_abilities =
            construct_u32_map(util::join(path, GENERAL_SPECIAL_ABILITY_DIR))?;
        let geode_rituals =
            construct_u32_map(util::join(path, GEODE_RITUAL_DIR))?;
        let hair_colors = construct_u32_map(util::join(path, HAIR_COLOR_DIR))?;
        let item_groups = construct_u32_map(util::join(path, ITEM_GROUP_DIR))?;
        let jester_tricks =
            construct_u32_map(util::join(path, JESTER_TRICK_DIR))?;
        let karma_special_abilities =
            construct_u32_map(util::join(path, KARMA_SPECIAL_ABILITY_DIR))?;
        let languages = construct_u32_map(util::join(path, LANGUAGE_DIR))?;
        let liturgical_chants =
            construct_u32_map(util::join(path, LITURGICAL_CHANT_DIR))?;
        let liturgical_chant_groups =
            construct_u32_map(util::join(path, LITURGICAL_CHANT_GROUP_DIR))?;
        let liturgical_style_special_abilities =
            construct_u32_map(
                util::join(path, LITURGICAL_STYLE_SPECIAL_ABILITY_DIR))?;
        let magical_dances =
            construct_u32_map(util::join(path, MAGICAL_DANCE_DIR))?;
        let magical_melodies =
            construct_u32_map(util::join(path, MAGICAL_MELODY_DIR))?;
        let magical_traditions =
            construct_u32_map(util::join(path, MAGICAL_TRADITION_DIR))?;
        let magical_special_abilities =
            construct_u32_map(util::join(path, MAGICAL_SPECIAL_ABILITY_DIR))?;
        let magic_style_special_abilities =
            construct_u32_map(
                util::join(path, MAGIC_STYLE_SPECIAL_ABILITY_DIR))?;
        let races = construct_u32_map(util::join(path, RACE_DIR))?;
        let reaches = construct_u32_map(util::join(path, REACH_DIR))?;
        let rituals = construct_u32_map(util::join(path, RITUAL_DIR))?;
        let scripts = construct_u32_map(util::join(path, SCRIPT_DIR))?;
        let skills = construct_u32_map(util::join(path, SKILL_DIR))?;
        let skill_groups =
            construct_u32_map(util::join(path, SKILL_GROUP_DIR))?;
        let skill_style_special_abilities =
            construct_u32_map(
                util::join(path, SKILL_STYLE_SPECIAL_ABILITY_DIR))?;
        let social_statuses =
            construct_u32_map(util::join(path, SOCIAL_STATUS_DIR))?;
        let spell_groups =
            construct_u32_map(util::join(path, SPELL_GROUP_DIR))?;
        let spells = construct_u32_map(util::join(path, SPELL_DIR))?;
        let subjects = construct_u32_map(util::join(path, SUBJECT_DIR))?;
        let tribes = construct_u32_map(util::join(path, TRIBE_DIR))?;
        let zibilja_rituals =
            construct_u32_map(util::join(path, ZIBILJA_RITUAL_DIR))?;
        let uis =
            construct_map(
                util::join(path, UI_DIR),
                |_: &UI, d| {
                    let os_file_name = d.file_name();
                    let file_name = os_file_name.to_str().unwrap();
                    let locale_name = file_name.split('.').next().unwrap();
                    String::from(locale_name)
                })?;

        Ok(OptolithData {
            advanced_combat_special_abilities,
            advanced_karma_special_abilities,
            advanced_magical_special_abilities,
            advanced_skill_special_abilities,
            advantages,
            ancestor_glyphs,
            animist_powers: animist_forces,
            arcane_bard_traditions,
            arcane_dancer_traditions,
            aspects,
            attributes,
            blessed_traditions,
            blessings,
            brawling_special_abilities,
            brews,
            ceremonies,
            combat_special_abilities,
            combat_style_special_abilities,
            command_special_abilities,
            conditions,
            curses,
            derived_characteristics,
            disadvantages,
            domination_rituals,
            elements,
            elven_magical_songs,
            experience_levels,
            eye_colors,
            general_special_abilities,
            geode_rituals,
            hair_colors,
            item_groups,
            jester_tricks,
            karma_special_abilities,
            languages,
            liturgical_chants,
            liturgical_chant_groups,
            liturgical_style_special_abilities,
            magical_dances,
            magical_melodies,
            magical_traditions,
            magical_special_abilities,
            magic_style_special_abilities,
            races,
            reaches,
            rituals,
            scripts,
            skills,
            skill_groups,
            skill_style_special_abilities,
            social_statuses,
            spell_groups,
            spells,
            subjects,
            tribes,
            zibilja_rituals,
            uis
        })
    }

    pub fn from_file(path: &str) -> OptolithDataResult<OptolithData> {
        util::from_compressed_file(path)
    }

    pub fn save_to_file(&self, path: &str) -> OptolithDataResult<()> {
        util::to_compressed_file(self, path)
    }

    pub fn get_advanced_combat_special_ability(&self, id: u32)
            -> Option<&AdvancedCombatSpecialAbility> {
        self.advanced_combat_special_abilities.get(&id)
    }

    pub fn get_advanced_karma_special_ability(&self, id: u32)
            -> Option<&AdvancedKarmaSpecialAbility> {
        self.advanced_karma_special_abilities.get(&id)
    }

    pub fn get_advanced_magical_special_ability(&self, id: u32)
            -> Option<&AdvancedMagicalSpecialAbility> {
        self.advanced_magical_special_abilities.get(&id)
    }

    pub fn get_advanced_skill_special_ability(&self, id: u32)
            -> Option<&AdvancedSkillSpecialAbility> {
        self.advanced_skill_special_abilities.get(&id)
    }

    pub fn get_advantage(&self, id: u32) -> Option<&Advantage> {
        self.advantages.get(&id)
    }

    pub fn get_ancestor_glyph(&self, id: u32) -> Option<&AncestorGlyph> {
        self.ancestor_glyphs.get(&id)
    }

    pub fn get_animist_power(&self, id: u32) -> Option<&AnimistPower> {
        self.animist_powers.get(&id)
    }

    pub fn get_arcane_bard_tradition(&self, id: u32)
            -> Option<&ArcaneBardTradition> {
        self.arcane_bard_traditions.get(&id)
    }

    pub fn get_arcane_dancer_tradition(&self, id: u32)
            -> Option<&ArcaneDancerTradition> {
        self.arcane_dancer_traditions.get(&id)
    }

    pub fn get_aspect(&self, id: u32) -> Option<&Aspect> {
        self.aspects.get(&id)
    }

    pub fn get_attribute(&self, id: u32) -> Option<&Attribute> {
        self.attributes.get(&id)
    }

    pub fn get_blessed_tradition(&self, id: u32) -> Option<&BlessedTradition> {
        self.blessed_traditions.get(&id)
    }

    pub fn get_blessing(&self, id: u32) -> Option<&Blessing> {
        self.blessings.get(&id)
    }

    pub fn get_brawling_special_ability(&self, id: u32)
            -> Option<&BrawlingSpecialAbility> {
        self.brawling_special_abilities.get(&id)
    }

    pub fn get_brew(&self, id: u32) -> Option<&Brew> {
        self.brews.get(&id)
    }

    pub fn get_ceremony(&self, id: u32) -> Option<&Ceremony> {
        self.ceremonies.get(&id)
    }

    pub fn get_combat_special_ability(&self, id: u32)
            -> Option<&CombatSpecialAbility> {
        self.combat_special_abilities.get(&id)
    }

    pub fn get_combat_style_special_ability(&self, id: u32)
            -> Option<&CombatStyleSpecialAbility> {
        self.combat_style_special_abilities.get(&id)
    }

    pub fn get_command_special_ability(&self, id: u32)
            -> Option<&CommandSpecialAbility> {
        self.command_special_abilities.get(&id)
    }

    pub fn get_condition(&self, id: u32) -> Option<&Condition> {
        self.conditions.get(&id)
    }

    pub fn get_curse(&self, id: u32) -> Option<&Curse> {
        self.curses.get(&id)
    }

    pub fn get_derived_characteristic(&self, id: u32)
            -> Option<&DerivedCharacteristic> {
        self.derived_characteristics.get(&id)
    }

    pub fn get_disadvantage(&self, id: u32) -> Option<&Disadvantage> {
        self.disadvantages.get(&id)
    }

    pub fn get_domination_ritual(&self, id: u32) -> Option<&DominationRitual> {
        self.domination_rituals.get(&id)
    }

    pub fn get_element(&self, id: u32) -> Option<&Element> {
        self.elements.get(&id)
    }

    pub fn get_elven_magical_song(&self, id: u32)
            -> Option<&ElvenMagicalSong> {
        self.elven_magical_songs.get(&id)
    }

    pub fn get_experience_level(&self, id: u32) -> Option<&ExperienceLevel> {
        self.experience_levels.get(&id)
    }

    pub fn get_eye_color(&self, id: u32) -> Option<&EyeColor> {
        self.eye_colors.get(&id)
    }

    pub fn get_general_special_ability(&self, id: u32)
            -> Option<&GeneralSpecialAbility> {
        self.general_special_abilities.get(&id)
    }

    pub fn get_geode_ritual(&self, id: u32) -> Option<&GeodeRitual> {
        self.geode_rituals.get(&id)
    }

    pub fn get_hair_color(&self, id: u32) -> Option<&HairColor> {
        self.hair_colors.get(&id)
    }

    pub fn get_item_group(&self, id: u32) -> Option<&ItemGroup> {
        self.item_groups.get(&id)
    }

    pub fn get_jester_trick(&self, id: u32) -> Option<&JesterTrick> {
        self.jester_tricks.get(&id)
    }

    pub fn get_karma_special_ability(&self, id: u32)
            -> Option<&KarmaSpecialAbility> {
        self.karma_special_abilities.get(&id)
    }

    pub fn get_language(&self, id: u32) -> Option<&Language> {
        self.languages.get(&id)
    }

    pub fn get_liturgical_chant(&self, id: u32) -> Option<&LiturgicalChant> {
        self.liturgical_chants.get(&id)
    }

    pub fn get_liturgical_chant_group(&self, id: u32)
            -> Option<&LiturgicalChantGroup> {
        self.liturgical_chant_groups.get(&id)
    }

    pub fn get_liturgical_style_special_ability(&self, id: u32)
            -> Option<&LiturgicalStyleSpecialAbility> {
        self.liturgical_style_special_abilities.get(&id)
    }

    pub fn get_magical_dance(&self, id: u32) -> Option<&MagicalDance> {
        self.magical_dances.get(&id)
    }

    pub fn get_magical_melody(&self, id: u32) -> Option<&MagicalMelody> {
        self.magical_melodies.get(&id)
    }

    pub fn get_magical_tradition(&self, id: u32) -> Option<&MagicalTradition> {
        self.magical_traditions.get(&id)
    }

    pub fn get_magical_special_ability(&self, id: u32)
            -> Option<&MagicalSpecialAbility> {
        self.magical_special_abilities.get(&id)
    }

    pub fn get_magic_style_special_ability(&self, id: u32)
            -> Option<&MagicStyleSpecialAbility> {
        self.magic_style_special_abilities.get(&id)
    }

    pub fn get_race(&self, id: u32) -> Option<&Race> {
        self.races.get(&id)
    }

    pub fn get_reach(&self, id: u32) -> Option<&Reach> {
        self.reaches.get(&id)
    }

    pub fn get_ritual(&self, id: u32) -> Option<&Ritual> {
        self.rituals.get(&id)
    }

    pub fn get_script(&self, id: u32) -> Option<&Script> {
        self.scripts.get(&id)
    }

    pub fn get_skill(&self, id: u32) -> Option<&Skill> {
        self.skills.get(&id)
    }

    pub fn get_skill_group(&self, id: u32) -> Option<&SkillGroup> {
        self.skill_groups.get(&id)
    }

    pub fn get_skill_style_special_ability(&self, id: u32)
            -> Option<&SkillStyleSpecialAbility> {
        self.skill_style_special_abilities.get(&id)
    }

    pub fn get_social_status(&self, id: u32) -> Option<&SocialStatus> {
        self.social_statuses.get(&id)
    }

    pub fn get_spell(&self, id: u32) -> Option<&Spell> {
        self.spells.get(&id)
    }

    pub fn get_spell_group(&self, id: u32) -> Option<&SpellGroup> {
        self.spell_groups.get(&id)
    }

    pub fn get_subject(&self, id: u32) -> Option<&Subject> {
        self.subjects.get(&id)
    }

    pub fn get_tribe(&self, id: u32) -> Option<&Tribe> {
        self.tribes.get(&id)
    }

    pub fn get_zibilja_ritual(&self, id: u32) -> Option<&ZibiljaRitual> {
        self.zibilja_rituals.get(&id)
    }

    pub fn get_ui_string(&self, locale: &str, id: &str) -> Option<&String> {
        self.uis.get(locale)
            .map(|ui| ui.get(id))
            .flatten()
    }

    pub fn get_as_translatable(&self, id: Id) -> Option<&dyn DynTranslatable> {
        let int_id = id.internal_id();

        match id.category() {
            Category::AdvancedCombatSpecialAbilities =>
                to_dyn(self.get_advanced_combat_special_ability(int_id)),
            Category::AdvancedKarmaSpecialAbilities =>
                to_dyn(self.get_advanced_karma_special_ability(int_id)),
            Category::AdvancedMagicalSpecialAbilities =>
                to_dyn(self.get_advanced_magical_special_ability(int_id)),
            Category::AdvancedSkillSpecialAbilities =>
                to_dyn(self.get_advanced_skill_special_ability(int_id)),
            Category::Advantages =>
                to_dyn(self.get_advantage(int_id)),
            Category::AncestorGlyphs =>
                to_dyn(self.get_ancestor_glyph(int_id)),
            Category::AnimistPowers =>
                to_dyn(self.get_animist_power(int_id)),
            Category::ArcaneBardTraditions =>
                to_dyn(self.get_arcane_bard_tradition(int_id)),
            Category::ArcaneDancerTraditions =>
                to_dyn(self.get_arcane_dancer_tradition(int_id)),
            Category::Aspects =>
                to_dyn(self.get_aspect(int_id)),
            Category::Attributes =>
                to_dyn(self.get_attribute(int_id)),
            Category::BlessedTraditions =>
                to_dyn(self.get_blessed_tradition(int_id)),
            Category::Blessings =>
                to_dyn(self.get_blessing(int_id)),
            Category::BrawlingSpecialAbilities =>
                to_dyn(self.get_brawling_special_ability(int_id)),
            Category::Brews =>
                to_dyn(self.get_brew(int_id)),
            Category::Ceremonies =>
                to_dyn(self.get_ceremony(int_id)),
            Category::CombatSpecialAbilities =>
                to_dyn(self.get_combat_special_ability(int_id)),
            Category::CombatStyleSpecialAbilities =>
                to_dyn(self.get_combat_style_special_ability(int_id)),
            Category::CommandSpecialAbilities =>
                to_dyn(self.get_command_special_ability(int_id)),
            Category::Conditions =>
                to_dyn(self.get_condition(int_id)),
            Category::Curses =>
                to_dyn(self.get_curse(int_id)),
            Category::DerivedCharacteristics =>
                to_dyn(self.get_derived_characteristic(int_id)),
            Category::Disadvantages =>
                to_dyn(self.get_disadvantage(int_id)),
            Category::DominationRituals =>
                to_dyn(self.get_domination_ritual(int_id)),
            Category::Elements =>
                to_dyn(self.get_element(int_id)),
            Category::ElvenMagicalSongs =>
                to_dyn(self.get_elven_magical_song(int_id)),
            Category::ExperienceLevels =>
                to_dyn(self.get_experience_level(int_id)),
            Category::EyeColors =>
                to_dyn(self.get_eye_color(int_id)),
            Category::GeneralSpecialAbilities =>
                to_dyn(self.get_general_special_ability(int_id)),
            Category::GeodeRituals =>
                to_dyn(self.get_geode_ritual(int_id)),
            Category::HairColors =>
                to_dyn(self.get_hair_color(int_id)),
            Category::ItemGroups =>
                to_dyn(self.get_item_group(int_id)),
            Category::JesterTricks =>
                to_dyn(self.get_jester_trick(int_id)),
            Category::KarmaSpecialAbilities =>
                to_dyn(self.get_karma_special_ability(int_id)),
            Category::Languages =>
                to_dyn(self.get_language(int_id)),
            Category::LiturgicalChants =>
                to_dyn(self.get_liturgical_chant(int_id)),
            Category::LiturgicalChantGroups =>
                to_dyn(self.get_liturgical_chant_group(int_id)),
            Category::LiturgicalStyleSpecialAbilities =>
                to_dyn(self.get_liturgical_style_special_ability(int_id)),
            Category::MagicalDances =>
                to_dyn(self.get_magical_dance(int_id)),
            Category::MagicalMelodies =>
                to_dyn(self.get_magical_melody(int_id)),
            Category::MagicalTraditions =>
                to_dyn(self.get_magical_tradition(int_id)),
            Category::MagicalSpecialAbilities =>
                to_dyn(self.get_magical_special_ability(int_id)),
            Category::MagicStyleSpecialAbilities =>
                to_dyn(self.get_magic_style_special_ability(int_id)),
            Category::Races =>
                to_dyn(self.get_race(int_id)),
            Category::Reaches =>
                to_dyn(self.get_reach(int_id)),
            Category::Rituals =>
                to_dyn(self.get_ritual(int_id)),
            Category::Scripts =>
                to_dyn(self.get_script(int_id)),
            Category::Skills =>
                to_dyn(self.get_skill(int_id)),
            Category::SkillGroups =>
                to_dyn(self.get_skill_group(int_id)),
            Category::SkillStyleSpecialAbilities =>
                to_dyn(self.get_skill_style_special_ability(int_id)),
            Category::SocialStatuses =>
                to_dyn(self.get_social_status(int_id)),
            Category::Spells =>
                to_dyn(self.get_spell(int_id)),
            Category::SpellGroups =>
                to_dyn(self.get_spell_group(int_id)),
            Category::Subjects =>
                to_dyn(self.get_subject(int_id)),
            Category::Tribes =>
                to_dyn(self.get_tribe(int_id)),
            Category::ZibiljaRituals =>
                to_dyn(self.get_zibilja_ritual(int_id)),
            _ => None // TODO update until all are implemented
        }
    }
}

// TODO why is this ridiculous thing needed?

fn to_dyn<T: Translatable>(t: Option<&T>) -> Option<&dyn DynTranslatable> {
    match t {
        Some(t) => Some(t),
        None => None
    }
}

/// A trait for localizations for entities. The minimal requirement for each
/// localization is to provide the name of its entity.
pub trait Localization {
    fn name(&self) -> &str;

    fn name_as_select_option(&self) -> &str {
        self.name()
    }
}

/// A [Localization] that consists only of a string.
#[derive(Deserialize, Serialize)]
pub struct SimpleLocalization {
    pub name: String
}

impl Localization for SimpleLocalization {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Localization for String {
    fn name(&self) -> &str {
        &self
    }
}

/// A map of language identifiers to [Localization]s of type `L`.
pub type Translations<L> = HashMap<String, L>;

pub type SimpleTranslations = Translations<SimpleLocalization>;

/// A trait for entities which are translatable, i.e. for which [Translation]s
/// of some [Localization] type `L` exist.
pub trait Translatable {
    type Localization: Localization;

    fn translations(&self) -> &Translations<Self::Localization>;

    fn translation(&self, locale: &str) -> Option<&Self::Localization> {
        self.translations().get(locale)
    }
}

pub trait DynTranslatable {
    fn translation(&self, locale: &str) -> Option<&dyn Localization>;
}

impl<T: Translatable> DynTranslatable for T {
    fn translation(&self, locale: &str) -> Option<&dyn Localization> {
        match Translatable::translation(self, locale) {
            Some(t) => Some(t),
            None => None
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Ids {
    Single(u32),
    List(Vec<u32>)
}
