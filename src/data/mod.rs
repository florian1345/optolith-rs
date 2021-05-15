use crate::data::academy::{Curriculum, Guideline, Influence};
use crate::data::activatable::character_trait::{
    Advantage,
    Disadvantage
};
use crate::data::activatable::special_ability::{
    SpecialAbilityGroup,
    ceremonial_item::CeremonialItemSpecialAbility,
    combat::{
        AdvancedCombatSpecialAbility,
        BrawlingSpecialAbility,
        CombatSpecialAbility,
        CombatStyleSpecialAbility,
        CommandSpecialAbility
    },
    enchantment::{
        AnimalShape,
        AnimalShapePath,
        AnimalShapeSize,
        ArcaneOrbEnchantment,
        AttireEnchantment,
        BowlEnchantment,
        CauldronEnchantment,
        ChronicleEnchantment,
        DaggerRitual,
        FoolsHatEnchantment,
        InstrumentEnchantment,
        Krallenkettenzauber,
        OrbEnchantment,
        RingEnchantment,
        SickleRitual,
        SpellSwordEnchantment,
        StaffEnchantment,
        ToyEnchantment,
        Trinkhornzauber,
        WandEnchantment,
        WeaponEnchantment
    },
    familiars_trick::FamiliarsTrick,
    gift::{
        LycantropicGift,
        PactCategory,
        PactGift,
        VampiricGift
    },
    non_profane::{
        AdvancedKarmaSpecialAbility,
        AdvancedMagicalSpecialAbility,
        AncestorGlyph,
        FamiliarSpecialAbility,
        KarmaSpecialAbility,
        LiturgicalStyleSpecialAbility,
        MagicalSpecialAbility,
        MagicStyleSpecialAbility,
        ProtectiveWardingCircleSpecialAbility,
        Sermon,
        Vision
    },
    ordinary::{
        FatePointSexSpecialAbility,
        FatePointSpecialAbility,
        GeneralSpecialAbility,
        SexSpecialAbility,
        SikaryanDrainSpecialAbility
    },
    skill::{
        AdvancedSkillSpecialAbility,
        SkillStyleSpecialAbility
    },
    trade_secret::TradeSecret,
    tradition::{
        ArcaneBardTradition,
        ArcaneDancerTradition,
        BlessedTradition,
        MagicalTradition,
        MagicalTraditionPlaceholder
    }
};
use crate::data::aspect::Aspect;
use crate::data::attribute::Attribute;
use crate::data::derived_characteristic::DerivedCharacteristic;
use crate::data::entity::Entity;
use crate::data::experience_level::ExperienceLevel;
use crate::data::item::{EquipmentPackage, ItemGroup};
use crate::data::language::{Language, Script};
use crate::data::package::culture::Culture;
use crate::data::package::profession::Profession;
use crate::data::patron::{Patron, PatronCategory};
use crate::data::personality_trait::PersonalityTrait;
use crate::data::property::Property;
use crate::data::publication::Publication;
use crate::data::race::Race;
use crate::data::rule::{FocusRule, OptionalRule};
use crate::data::service::Service;
use crate::data::sex::SexPractice;
use crate::data::simple::{
    AnimalType,
    ArmorType,
    Brew,
    CombatSpecialAbilityGroup,
    CombatTechniqueGroup,
    Element,
    EyeColor,
    HairColor,
    LiturgicalChantGroup,
    Reach,
    Region,
    SocialStatus,
    SpellGroup,
    Subject,
    Tribe
};
use crate::data::skill::{Skill, SkillGroup};
use crate::data::skill::combat::{MeleeCombatTechnique, RangedCombatTechnique};
use crate::data::skill::non_profane::karmal::{
    Blessing,
    Ceremony,
    LiturgicalChant
};
use crate::data::skill::non_profane::magical::{
    AnimistPower,
    Cantrip,
    Curse,
    DominationRitual,
    ElvenMagicalSong,
    GeodeRitual,
    JesterTrick,
    MagicalDance,
    MagicalMelody,
    MagicalRune,
    Ritual,
    Spell,
    ZibiljaRitual
};
use crate::data::status_effect::{
    AnimalDisease,
    Condition,
    Disease,
    Poison,
    State
};
use crate::id::{Category, Id, Identifiable};
use crate::error::OptolithDataResult;
use crate::util;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::DirEntry;
use std::hash::Hash;

pub mod academy;
pub mod activatable;
pub mod aspect;
pub mod attribute;
pub mod derived_characteristic;
pub mod effects_localization;
pub mod entity;
pub mod errata;
pub mod experience_level;
pub mod item;
pub mod language;
pub mod package;
pub mod patron;
pub mod personality_trait;
pub mod prerequisite;
pub mod property;
pub mod publication;
pub mod race;
pub mod rule;
pub mod service;
pub mod sex;
pub mod simple;
pub mod skill;
pub mod src;
pub mod status_effect;

const ADVANCED_COMBAT_SPECIAL_ABILITY_DIR: &str =
    "AdvancedCombatSpecialAbilities";
const ADVANCED_KARMA_SPECIAL_ABILITY_DIR: &str =
    "AdvancedKarmaSpecialAbilities";
const ADVANCED_MAGICAL_SPECIAL_ABILITY_DIR: &str =
    "AdvancedMagicalSpecialAbilities";
const ADVANCED_SKILL_SPECIAL_ABILITY_DIR: &str =
    "AdvancedSkillSpecialAbilities";
const ADVANTAGE_DIR: &str = "Advantages";
const ANCESTOR_GLYPH_DIR: &str = "AncestorGlyphs";
const ANIMAL_DISEASE_DIR: &str = "AnimalDiseases";
const ANIMAL_SHAPE_DIR: &str = "AnimalShapes";
const ANIMAL_SHAPE_PATH_DIR: &str = "AnimalShapePaths";
const ANIMAL_SHAPE_SIZE_DIR: &str = "AnimalShapeSizes";
const ANIMAL_TYPE_DIR: &str = "AnimalTypes";
const ANIMIST_POWER_DIR: &str = "AnimistPowers";
const ARCANE_BARD_TRADITION_DIR: &str = "ArcaneBardTraditions";
const ARCANE_DANCER_TRADITION_DIR: &str = "ArcaneDancerTraditions";
const ARCANE_ORB_ENCHANTMENT_DIR: &str = "ArcaneOrbEnchantments";
const ARMOR_TYPE_DIR: &str = "ArmorTypes";
const ASPECT_DIR: &str = "Aspects";
const ATTIRE_ENCHANTMENT_DIR: &str = "AttireEnchantments";
const ATTRIBUTE_DIR: &str = "Attributes";
const BLESSED_TRADITION_DIR: &str = "BlessedTraditions";
const BLESSING_DIR: &str = "Blessings";
const BOWL_ENCHANTMENT_DIR: &str = "BowlEnchantments";
const BRAWLING_SPECIAL_ABILITY_DIR: &str = "BrawlingSpecialAbilities";
const BREW_DIR: &str = "Brews";
const CANTRIP_DIR: &str = "Cantrips";
const CAULDRON_ENCHANTMENT_DIR: &str = "CauldronEnchantments";
const CEREMONIAL_ITEM_SPECIAL_ABILITY_DIR: &str =
    "CeremonialItemSpecialAbilities";
const CEREMONY_DIR: &str = "Ceremonies";
const CHRONICLE_ENCHANTMENT_DIR: &str = "ChronicleEnchantments";
const COMBAT_SPECIAL_ABILITY_DIR: &str = "CombatSpecialAbilities";
const COMBAT_SPECIAL_ABILITY_GROUP_DIR: &str = "CombatSpecialAbilityGroups";
const COMBAT_STYLE_SPECIAL_ABILITY_DIR: &str = "CombatStyleSpecialAbilities";
const COMBAT_TECHNIQUE_GROUP_DIR: &str = "CombatTechniqueGroups";
const COMMAND_SPECIAL_ABILITY_DIR: &str = "CommandSpecialAbilities";
const CONDITION_DIR: &str = "Conditions";
const CULTURE_DIR: &str = "Cultures";
// const CURRICULUM_DIR: &str = "Curricula";
const CURSE_DIR: &str = "Curses";
const DAGGER_RITUAL_DIR: &str = "DaggerRituals";
const DERIVED_CHARACTERISTIC_DIR: &str = "DerivedCharacteristics";
const DISADVANTAGE_DIR: &str = "Disadvantages";
const DISEASE_DIR: &str = "Diseases";
const DOMINATION_RITUAL_DIR: &str = "DominationRituals";
const ELEMENT_DIR: &str = "Elements";
const ELVEN_MAGICAL_SONG_DIR: &str = "ElvenMagicalSongs";
const EQUIPMENT_PACKAGE_DIR: &str = "EquipmentPackages";
const EXPERIENCE_LEVEL_DIR: &str = "ExperienceLevels";
const EYE_COLOR_DIR: &str = "EyeColors";
const FAMILIAR_SPECIAL_ABILITY_DIR: &str = "FamiliarSpecialAbilities";
const FAMILIARS_TRICK_DIR: &str = "FamiliarsTricks";
const FATE_POINT_SEX_SPECIAL_ABILITY_DIR: &str =
    "FatePointSexSpecialAbilities";
const FATE_POINT_SPECIAL_ABILITY_DIR: &str = "FatePointSpecialAbilities";
const FOCUS_RULE_DIR: &str = "FocusRules";
const FOOLS_HAT_ENCHANTMENT_DIR: &str = "FoolsHatEnchantments";
const GENERAL_SPECIAL_ABILITY_DIR: &str = "GeneralSpecialAbilities";
const GEODE_RITUAL_DIR: &str = "GeodeRituals";
const GUIDELINE_DIR: &str = "Guidelines";
const HAIR_COLOR_DIR: &str = "HairColors";
const INFLUENCE_DIR: &str = "Influences";
const INSTRUMENT_ENCHANTMENT_DIR: &str = "InstrumentEnchantments";
const ITEM_GROUP_DIR: &str = "ItemGroups";
const JESTER_TRICK_DIR: &str = "JesterTricks";
const KARMA_SPECIAL_ABILITY_DIR: &str = "KarmaSpecialAbilities";
const KRALLENKETTENZAUBER_DIR: &str = "Krallenkettenzauber";
const LANGUAGE_DIR: &str = "Languages";
const LITURGICAL_CHANT_DIR: &str = "LiturgicalChants";
const LITURGICAL_CHANT_GROUP_DIR: &str = "LiturgicalChantGroups";
const LITURGICAL_STYLE_SPECIAL_ABILITY_DIR: &str =
    "LiturgicalStyleSpecialAbilities";
const LYCANTROPIC_GIFT_DIR: &str = "LycantropicGifts";
const MAGICAL_DANCE_DIR: &str = "MagicalDances";
const MAGICAL_MELODY_DIR: &str = "MagicalMelodies";
const MAGICAL_RUNE_DIR: &str = "MagicalRunes";
const MAGICAL_SPECIAL_ABILITY_DIR: &str = "MagicalSpecialAbilities";
const MAGICAL_TRADITION_DIR: &str = "MagicalTraditions";
const MAGICAL_TRADITION_PLACEHOLDER_DIR: &str = "MagicalTraditionPlaceholders";
const MAGIC_STYLE_SPECIAL_ABILITY_DIR: &str = "MagicStyleSpecialAbilities";
const MELEE_COMBAT_TECHNIQUE_DIR: &str = "MeleeCombatTechniques";
const OPTIONAL_RULE_DIR: &str = "OptionalRules";
const ORB_ENCHANTMENT_DIR: &str = "OrbEnchantments";
const PACT_CATEGORY_DIR: &str = "PactCategories";
const PACT_GIFT_DIR: &str = "PactGifts";
const PATRON_DIR: &str = "Patrons";
const PATRON_CATEGORY_DIR: &str = "PatronCategories";
const PERSONALITY_TRAIT_DIR: &str = "PersonalityTraits";
const POISON_DIR: &str = "Poisons";
// const PROFESSION_DIR: &str = "Professions";
const PROPERTY_DIR: &str = "Properties";
const PROTECTIVE_WARDING_CIRCLE_SPECIAL_ABILITY_DIR: &str =
    "ProtectiveWardingCircleSpecialAbilities";
const PUBLICATION_DIR: &str = "Publications";
const RACE_DIR: &str = "Races";
const RANGED_COMBAT_TECHNIQUE_DIR: &str = "RangedCombatTechniques";
const REACH_DIR: &str = "Reaches";
const REGION_DIR: &str = "Regions";
const RING_ENCHANTMENT_DIR: &str = "RingEnchantments";
const RITUAL_DIR: &str = "Rituals";
const SCRIPT_DIR: &str = "Scripts";
const SERMON_DIR: &str = "Sermons";
const SERVICE_DIR: &str = "Services";
const SEX_PRACTICE_DIR: &str = "SexPractices";
const SEX_SPECIAL_ABILITY_DIR: &str = "SexSpecialAbilities";
const SICKLE_RITUAL_DIR: &str = "SickleRituals";
const SIKARYAN_DRAIN_SPECIAL_ABILITY_DIR: &str =
    "SikaryanDrainSpecialAbilities";
const SKILL_DIR: &str = "Skills";
const SKILL_GROUP_DIR: &str = "SkillGroups";
const SKILL_STYLE_SPECIAL_ABILITY_DIR: &str = "SkillStyleSpecialAbilities";
const SOCIAL_STATUS_DIR: &str = "SocialStatuses";
const SPECIAL_ABILITY_GROUP_DIR: &str = "SpecialAbilityGroups";
const SPELL_DIR: &str = "Spells";
const SPELL_GROUP_DIR: &str = "SpellGroups";
const SPELL_SWORD_ENCHANTMENT_DIR: &str = "SpellSwordEnchantments";
const STAFF_ENCHANTMENT_DIR: &str = "StaffEnchantments";
const STATE_DIR: &str = "States";
const SUBJECT_DIR: &str = "Subjects";
const TOY_ENCHANTMENT_DIR: &str = "ToyEnchantments";
const TRADE_SECRET_DIR: &str = "TradeSecrets";
const TRIBE_DIR: &str = "Tribes";
const TRINKHORNZAUBER_DIR: &str = "Trinkhornzauber";
const UI_DIR: &str = "UI";
const VAMPIRIC_GIFT_DIR: &str = "VampiricGifts";
const VISION_DIR: &str = "Visions";
const WAND_ENCHANTMENT_DIR: &str = "WandEnchantments";
const WEAPON_ENCHANTMENT_DIR: &str = "WeaponEnchantments";
const ZIBILJA_RITUAL_DIR: &str = "ZibiljaRituals";

const EXCLUDED_FILES: &[&str] = &[
    "25_Waldschrate.yml",
    "5_Toad-Poison.yml",
    "53_Mond.yml",
    "128_Keulenweihe.yml",
    "129_Keulenwurf.yml"
];

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum UIValue {
    Simple(String),
    SubMap(HashMap<String, String>)
}

type UI = HashMap<String, UIValue>;

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
    animal_diseases: IdMap<AnimalDisease>,
    animal_shapes: IdMap<AnimalShape>,
    animal_shape_paths: IdMap<AnimalShapePath>,
    animal_shape_sizes: IdMap<AnimalShapeSize>,
    animal_types: IdMap<AnimalType>,
    animist_powers: IdMap<AnimistPower>,
    arcane_bard_traditions: IdMap<ArcaneBardTradition>,
    arcane_dancer_traditions: IdMap<ArcaneDancerTradition>,
    arcane_orb_enchantments: IdMap<ArcaneOrbEnchantment>,
    armor_types: IdMap<ArmorType>,
    aspects: IdMap<Aspect>,
    attire_enchantments: IdMap<AttireEnchantment>,
    attributes: IdMap<Attribute>,
    blessed_traditions: IdMap<BlessedTradition>,
    blessings: IdMap<Blessing>,
    bowl_enchantments: IdMap<BowlEnchantment>,
    brawling_special_abilities: IdMap<BrawlingSpecialAbility>,
    brews: IdMap<Brew>,
    cantrips: IdMap<Cantrip>,
    cauldron_enchantments: IdMap<CauldronEnchantment>,
    ceremonial_item_special_abilities: IdMap<CeremonialItemSpecialAbility>,
    ceremonies: IdMap<Ceremony>,
    chronicle_enchantments: IdMap<ChronicleEnchantment>,
    combat_special_abilities: IdMap<CombatSpecialAbility>,
    combat_special_ability_groups: IdMap<CombatSpecialAbilityGroup>,
    combat_style_special_abilities: IdMap<CombatStyleSpecialAbility>,
    combat_technique_groups: IdMap<CombatTechniqueGroup>,
    command_special_abilities: IdMap<CommandSpecialAbility>,
    conditions: IdMap<Condition>,
    cultures: IdMap<Culture>,
    curricula: IdMap<Curriculum>,
    curses: IdMap<Curse>,
    dagger_rituals: IdMap<DaggerRitual>,
    derived_characteristics: IdMap<DerivedCharacteristic>,
    disadvantages: IdMap<Disadvantage>,
    diseases: IdMap<Disease>,
    domination_rituals: IdMap<DominationRitual>,
    elements: IdMap<Element>,
    elven_magical_songs: IdMap<ElvenMagicalSong>,
    equipment_packages: IdMap<EquipmentPackage>,
    experience_levels: IdMap<ExperienceLevel>,
    eye_colors: IdMap<EyeColor>,
    familiar_special_abilities: IdMap<FamiliarSpecialAbility>,
    familiars_tricks: IdMap<FamiliarsTrick>,
    fate_point_sex_special_abilities: IdMap<FatePointSexSpecialAbility>,
    fate_point_special_abilities: IdMap<FatePointSpecialAbility>,
    focus_rules: IdMap<FocusRule>,
    fools_hat_enchantments: IdMap<FoolsHatEnchantment>,
    general_special_abilities: IdMap<GeneralSpecialAbility>,
    geode_rituals: IdMap<GeodeRitual>,
    guidelines: IdMap<Guideline>,
    hair_colors: IdMap<HairColor>,
    influences: IdMap<Influence>,
    instrument_enchantments: IdMap<InstrumentEnchantment>,
    item_groups: IdMap<ItemGroup>,
    jester_tricks: IdMap<JesterTrick>,
    karma_special_abilities: IdMap<KarmaSpecialAbility>,
    krallenkettenzauber: IdMap<Krallenkettenzauber>,
    languages: IdMap<Language>,
    liturgical_chants: IdMap<LiturgicalChant>,
    liturgical_chant_groups: IdMap<LiturgicalChantGroup>,
    liturgical_style_special_abilities: IdMap<LiturgicalStyleSpecialAbility>,
    lycantropic_gifts: IdMap<LycantropicGift>,
    magical_dances: IdMap<MagicalDance>,
    magical_melodies: IdMap<MagicalMelody>,
    magical_runes: IdMap<MagicalRune>,
    magical_special_abilities: IdMap<MagicalSpecialAbility>,
    magical_traditions: IdMap<MagicalTradition>,
    magical_tradition_placeholders: IdMap<MagicalTraditionPlaceholder>,
    magic_style_special_abilities: IdMap<MagicStyleSpecialAbility>,
    melee_combat_techniques: IdMap<MeleeCombatTechnique>,
    optional_rules: IdMap<OptionalRule>,
    orb_enchantments: IdMap<OrbEnchantment>,
    pact_categories: IdMap<PactCategory>,
    pact_gifts: IdMap<PactGift>,
    patrons: IdMap<Patron>,
    patron_categories: IdMap<PatronCategory>,
    personality_traits: IdMap<PersonalityTrait>,
    poisons: IdMap<Poison>,
    professions: IdMap<Profession>,
    properties: IdMap<Property>,
    protective_warding_circle_special_abilities:
        IdMap<ProtectiveWardingCircleSpecialAbility>,
    publications: IdMap<Publication>,
    races: IdMap<Race>,
    ranged_combat_techniques: IdMap<RangedCombatTechnique>,
    reaches: IdMap<Reach>,
    regions: IdMap<Region>,
    ring_enchantments: IdMap<RingEnchantment>,
    rituals: IdMap<Ritual>,
    scripts: IdMap<Script>,
    sermons: IdMap<Sermon>,
    services: IdMap<Service>,
    sex_practices: IdMap<SexPractice>,
    sex_special_abilities: IdMap<SexSpecialAbility>,
    sickle_rituals: IdMap<SickleRitual>,
    sikaryan_drain_special_abilities: IdMap<SikaryanDrainSpecialAbility>,
    skills: IdMap<Skill>,
    skill_groups: IdMap<SkillGroup>,
    skill_style_special_abilities: IdMap<SkillStyleSpecialAbility>,
    social_statuses: IdMap<SocialStatus>,
    special_ability_groups: IdMap<SpecialAbilityGroup>,
    spells: IdMap<Spell>,
    spell_groups: IdMap<SpellGroup>,
    spell_sword_enchantments: IdMap<SpellSwordEnchantment>,
    staff_enchantments: IdMap<StaffEnchantment>,
    states: IdMap<State>,
    subjects: IdMap<Subject>,
    toy_enchantments: IdMap<ToyEnchantment>,
    trade_secrets: IdMap<TradeSecret>,
    tribes: IdMap<Tribe>,
    trinkhornzauber: IdMap<Trinkhornzauber>,
    vampiric_gifts: IdMap<VampiricGift>,
    visions: IdMap<Vision>,
    wand_enchantments: IdMap<WandEnchantment>,
    weapon_enchantments: IdMap<WeaponEnchantment>,
    zibilja_rituals: IdMap<ZibiljaRitual>,
    uis: HashMap<String, UI>
}

fn is_placeholder(path: &str) -> bool {
    // TODO remove once sorted out
    if EXCLUDED_FILES.contains(&path) {
        return true;
    }

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

struct IdMapBuilder<'a> {
    path: &'a str
}

impl<'a> IdMapBuilder<'a> {
    fn map<K, V>(&self, dir_name: &str,
        placeholder_predicate: impl Fn(&DirEntry) -> bool,
        key_builder: impl Fn(&V, &DirEntry) -> K)
        -> OptolithDataResult<HashMap<K, V>>
    where
        for<'de> V : Deserialize<'de>,
        K: Eq + Hash
    {
        let mut map: HashMap<K, V> = HashMap::new();
        let dir = util::join(self.path, dir_name);
    
        for file in util::read_dir(&dir)? {
            let file = file?;
    
            // TODO remove once a more permanent solution for placeholders has been
            // found
    
            if placeholder_predicate(&file) {
                continue;
            }
    
            let object: V = util::deserialize_yaml_file(&file.path())?;
            let id = key_builder(&object, &file);
            map.insert(id, object);
        }
    
        Ok(map)
    }

    fn map_u32<V>(&self, dir_name: &str) -> OptolithDataResult<IdMap<V>>
    where
        for<'de> V : Deserialize<'de> + Identifiable
    {
        self.map(dir_name, |d| is_placeholder(d.file_name().to_str().unwrap()),
            |v: &V, _| v.id().internal_id())
    }
}

impl OptolithData {
    pub fn from_directory(path: &str) -> OptolithDataResult<OptolithData> {
        let builder = IdMapBuilder {
            path
        };

        let advanced_combat_special_abilities =
            builder.map_u32(ADVANCED_COMBAT_SPECIAL_ABILITY_DIR)?;
        let advanced_karma_special_abilities =
            builder.map_u32(ADVANCED_KARMA_SPECIAL_ABILITY_DIR)?;
        let advanced_magical_special_abilities =
            builder.map_u32(ADVANCED_MAGICAL_SPECIAL_ABILITY_DIR)?;
        let advanced_skill_special_abilities =
            builder.map_u32(ADVANCED_SKILL_SPECIAL_ABILITY_DIR)?;
        let advantages = builder.map_u32(ADVANTAGE_DIR)?;
        let ancestor_glyphs = builder.map_u32(ANCESTOR_GLYPH_DIR)?;
        let animal_diseases = builder.map_u32(ANIMAL_DISEASE_DIR)?;
        let animal_shapes = builder.map_u32(ANIMAL_SHAPE_DIR)?;
        let animal_shape_paths = builder.map_u32(ANIMAL_SHAPE_PATH_DIR)?;
        let animal_shape_sizes = builder.map_u32(ANIMAL_SHAPE_SIZE_DIR)?;
        let animal_types = builder.map_u32(ANIMAL_TYPE_DIR)?;
        let animist_powers = builder.map_u32(ANIMIST_POWER_DIR)?;
        let arcane_bard_traditions =
            builder.map_u32(ARCANE_BARD_TRADITION_DIR)?;
        let arcane_dancer_traditions =
            builder.map_u32(ARCANE_DANCER_TRADITION_DIR)?;
        let arcane_orb_enchantments =
            builder.map_u32(ARCANE_ORB_ENCHANTMENT_DIR)?;
        let armor_types = builder.map_u32(ARMOR_TYPE_DIR)?;
        let aspects = builder.map_u32(ASPECT_DIR)?;
        let attire_enchantments = builder.map_u32(ATTIRE_ENCHANTMENT_DIR)?;
        let attributes = builder.map_u32(ATTRIBUTE_DIR)?;
        let blessed_traditions = builder.map_u32(BLESSED_TRADITION_DIR)?;
        let blessings = builder.map_u32(BLESSING_DIR)?;
        let brawling_special_abilities =
            builder.map_u32(BRAWLING_SPECIAL_ABILITY_DIR)?;
        let bowl_enchantments = builder.map_u32(BOWL_ENCHANTMENT_DIR)?;
        let brews = builder.map_u32(BREW_DIR)?;
        let cantrips = builder.map_u32(CANTRIP_DIR)?;
        let cauldron_enchantments = builder.map_u32(CAULDRON_ENCHANTMENT_DIR)?;
        let ceremonial_item_special_abilities =
            builder.map_u32(CEREMONIAL_ITEM_SPECIAL_ABILITY_DIR)?;
        let ceremonies = builder.map_u32(CEREMONY_DIR)?;
        let chronicle_enchantments =
            builder.map_u32(CHRONICLE_ENCHANTMENT_DIR)?;
        let combat_special_abilities =
            builder.map_u32(COMBAT_SPECIAL_ABILITY_DIR)?;
        let combat_special_ability_groups =
            builder.map_u32(COMBAT_SPECIAL_ABILITY_GROUP_DIR)?;
        let combat_style_special_abilities =
            builder.map_u32(COMBAT_STYLE_SPECIAL_ABILITY_DIR)?;
        let combat_technique_groups =
            builder.map_u32(COMBAT_TECHNIQUE_GROUP_DIR)?;
        let command_special_abilities =
            builder.map_u32(COMMAND_SPECIAL_ABILITY_DIR)?;
        let conditions = builder.map_u32(CONDITION_DIR)?;
        let cultures = builder.map_u32(CULTURE_DIR)?;
        // TODO reactivate once errors are gone
        let curricula = HashMap::new(); //builder.map_u32(CURRICULUM_DIR)?;
        let curses = builder.map_u32(CURSE_DIR)?;
        let dagger_rituals = builder.map_u32(DAGGER_RITUAL_DIR)?;
        let derived_characteristics =
            builder.map_u32(DERIVED_CHARACTERISTIC_DIR)?;
        let disadvantages = builder.map_u32(DISADVANTAGE_DIR)?;
        let diseases = builder.map_u32(DISEASE_DIR)?;
        let domination_rituals = builder.map_u32(DOMINATION_RITUAL_DIR)?;
        let elements = builder.map_u32(ELEMENT_DIR)?;
        let elven_magical_songs = builder.map_u32(ELVEN_MAGICAL_SONG_DIR)?;
        let equipment_packages = builder.map_u32(EQUIPMENT_PACKAGE_DIR)?;
        let experience_levels = builder.map_u32(EXPERIENCE_LEVEL_DIR)?;
        let eye_colors = builder.map_u32(EYE_COLOR_DIR)?;
        let familiar_special_abilities =
            builder.map_u32(FAMILIAR_SPECIAL_ABILITY_DIR)?;
        let familiars_tricks = builder.map_u32(FAMILIARS_TRICK_DIR)?;
        let fate_point_sex_special_abilities =
            builder.map_u32(FATE_POINT_SEX_SPECIAL_ABILITY_DIR)?;
        let fate_point_special_abilities =
            builder.map_u32(FATE_POINT_SPECIAL_ABILITY_DIR)?;
        let focus_rules = builder.map_u32(FOCUS_RULE_DIR)?;
        let fools_hat_enchantments =
            builder.map_u32(FOOLS_HAT_ENCHANTMENT_DIR)?;
        let general_special_abilities =
            builder.map_u32(GENERAL_SPECIAL_ABILITY_DIR)?;
        let geode_rituals = builder.map_u32(GEODE_RITUAL_DIR)?;
        let guidelines = builder.map_u32(GUIDELINE_DIR)?;
        let hair_colors = builder.map_u32(HAIR_COLOR_DIR)?;
        let influences = builder.map_u32(INFLUENCE_DIR)?;
        let instrument_enchantments =
            builder.map_u32(INSTRUMENT_ENCHANTMENT_DIR)?;
        let item_groups = builder.map_u32(ITEM_GROUP_DIR)?;
        let jester_tricks = builder.map_u32(JESTER_TRICK_DIR)?;
        let karma_special_abilities =
            builder.map_u32(KARMA_SPECIAL_ABILITY_DIR)?;
        let krallenkettenzauber = builder.map_u32(KRALLENKETTENZAUBER_DIR)?;
        let languages = builder.map_u32(LANGUAGE_DIR)?;
        let liturgical_chants = builder.map_u32(LITURGICAL_CHANT_DIR)?;
        let liturgical_chant_groups =
            builder.map_u32(LITURGICAL_CHANT_GROUP_DIR)?;
        let liturgical_style_special_abilities =
            builder.map_u32(LITURGICAL_STYLE_SPECIAL_ABILITY_DIR)?;
        let lycantropic_gifts = builder.map_u32(LYCANTROPIC_GIFT_DIR)?;
        let magical_dances = builder.map_u32(MAGICAL_DANCE_DIR)?;
        let magical_melodies = builder.map_u32(MAGICAL_MELODY_DIR)?;
        let magical_runes = builder.map_u32(MAGICAL_RUNE_DIR)?;
        let magical_special_abilities =
            builder.map_u32(MAGICAL_SPECIAL_ABILITY_DIR)?;
        let magical_traditions = builder.map_u32(MAGICAL_TRADITION_DIR)?;
        let magical_tradition_placeholders =
            builder.map_u32(MAGICAL_TRADITION_PLACEHOLDER_DIR)?;
        let magic_style_special_abilities =
            builder.map_u32(MAGIC_STYLE_SPECIAL_ABILITY_DIR)?;
        let melee_combat_techniques =
            builder.map_u32(MELEE_COMBAT_TECHNIQUE_DIR)?;
        let optional_rules = builder.map_u32(OPTIONAL_RULE_DIR)?;
        let orb_enchantments = builder.map_u32(ORB_ENCHANTMENT_DIR)?;
        let pact_categories = builder.map_u32(PACT_CATEGORY_DIR)?;
        let pact_gifts = builder.map_u32(PACT_GIFT_DIR)?;
        let patrons = builder.map_u32(PATRON_DIR)?;
        let patron_categories = builder.map_u32(PATRON_CATEGORY_DIR)?;
        let personality_traits = builder.map_u32(PERSONALITY_TRAIT_DIR)?;
        let poisons = builder.map_u32(POISON_DIR)?;
        // TODO reactivate once errors are gone
        let professions = HashMap::new(); //builder.map_u32(PROFESSION_DIR)?;
        let properties = builder.map_u32(PROPERTY_DIR)?;
        let protective_warding_circle_special_abilities =
            builder.map_u32(PROTECTIVE_WARDING_CIRCLE_SPECIAL_ABILITY_DIR)?;
        let publications = builder.map_u32(PUBLICATION_DIR)?;
        let races = builder.map_u32(RACE_DIR)?;
        let ranged_combat_techniques =
            builder.map_u32(RANGED_COMBAT_TECHNIQUE_DIR)?;
        let reaches = builder.map_u32(REACH_DIR)?;
        let regions = builder.map_u32(REGION_DIR)?;
        let ring_enchantments = builder.map_u32(RING_ENCHANTMENT_DIR)?;
        let rituals = builder.map_u32(RITUAL_DIR)?;
        let scripts = builder.map_u32(SCRIPT_DIR)?;
        let sex_practices = builder.map_u32(SEX_PRACTICE_DIR)?;
        let sex_special_abilities = builder.map_u32(SEX_SPECIAL_ABILITY_DIR)?;
        let sermons = builder.map_u32(SERMON_DIR)?;
        let services = builder.map_u32(SERVICE_DIR)?;
        let sickle_rituals = builder.map_u32(SICKLE_RITUAL_DIR)?;
        let sikaryan_drain_special_abilities =
            builder.map_u32(SIKARYAN_DRAIN_SPECIAL_ABILITY_DIR)?;
        let skills = builder.map_u32(SKILL_DIR)?;
        let skill_groups = builder.map_u32(SKILL_GROUP_DIR)?;
        let skill_style_special_abilities =
            builder.map_u32(SKILL_STYLE_SPECIAL_ABILITY_DIR)?;
        let social_statuses = builder.map_u32(SOCIAL_STATUS_DIR)?;
        let special_ability_groups = builder.map_u32(SPECIAL_ABILITY_GROUP_DIR)?;
        let spell_groups = builder.map_u32(SPELL_GROUP_DIR)?;
        let spell_sword_enchantments =
            builder.map_u32(SPELL_SWORD_ENCHANTMENT_DIR)?;
        let spells = builder.map_u32(SPELL_DIR)?;
        let staff_enchantments = builder.map_u32(STAFF_ENCHANTMENT_DIR)?;
        let states = builder.map_u32(STATE_DIR)?;
        let subjects = builder.map_u32(SUBJECT_DIR)?;
        let toy_enchantments = builder.map_u32(TOY_ENCHANTMENT_DIR)?;
        let trade_secrets = builder.map_u32(TRADE_SECRET_DIR)?;
        let tribes = builder.map_u32(TRIBE_DIR)?;
        let trinkhornzauber = builder.map_u32(TRINKHORNZAUBER_DIR)?;
        let vampiric_gifts = builder.map_u32(VAMPIRIC_GIFT_DIR)?;
        let visions = builder.map_u32(VISION_DIR)?;
        let wand_enchantments = builder.map_u32(WAND_ENCHANTMENT_DIR)?;
        let weapon_enchantments = builder.map_u32(WEAPON_ENCHANTMENT_DIR)?;
        let zibilja_rituals = builder.map_u32(ZIBILJA_RITUAL_DIR)?;
        let uis =
            builder.map(UI_DIR,
                |_| false,
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
            animal_diseases,
            animal_shapes,
            animal_shape_paths,
            animal_shape_sizes,
            animal_types,
            animist_powers,
            arcane_bard_traditions,
            arcane_dancer_traditions,
            arcane_orb_enchantments,
            armor_types,
            aspects,
            attire_enchantments,
            attributes,
            blessed_traditions,
            blessings,
            bowl_enchantments,
            brawling_special_abilities,
            brews,
            cantrips,
            cauldron_enchantments,
            ceremonial_item_special_abilities,
            ceremonies,
            chronicle_enchantments,
            combat_special_abilities,
            combat_special_ability_groups,
            combat_style_special_abilities,
            combat_technique_groups,
            command_special_abilities,
            conditions,
            cultures,
            curricula,
            curses,
            dagger_rituals,
            derived_characteristics,
            disadvantages,
            diseases,
            domination_rituals,
            elements,
            elven_magical_songs,
            equipment_packages,
            experience_levels,
            eye_colors,
            familiar_special_abilities,
            familiars_tricks,
            fate_point_sex_special_abilities,
            fate_point_special_abilities,
            focus_rules,
            fools_hat_enchantments,
            general_special_abilities,
            geode_rituals,
            guidelines,
            hair_colors,
            influences,
            instrument_enchantments,
            item_groups,
            jester_tricks,
            karma_special_abilities,
            krallenkettenzauber,
            languages,
            liturgical_chants,
            liturgical_chant_groups,
            liturgical_style_special_abilities,
            lycantropic_gifts,
            magical_dances,
            magical_melodies,
            magical_runes,
            magical_special_abilities,
            magical_traditions,
            magical_tradition_placeholders,
            magic_style_special_abilities,
            melee_combat_techniques,
            optional_rules,
            orb_enchantments,
            pact_categories,
            pact_gifts,
            patrons,
            patron_categories,
            personality_traits,
            poisons,
            professions,
            properties,
            protective_warding_circle_special_abilities,
            publications,
            races,
            ranged_combat_techniques,
            reaches,
            regions,
            ring_enchantments,
            rituals,
            scripts,
            sermons,
            services,
            sex_practices,
            sex_special_abilities,
            sickle_rituals,
            sikaryan_drain_special_abilities,
            skills,
            skill_groups,
            skill_style_special_abilities,
            social_statuses,
            special_ability_groups,
            spell_groups,
            spell_sword_enchantments,
            spells,
            staff_enchantments,
            states,
            subjects,
            toy_enchantments,
            trade_secrets,
            tribes,
            trinkhornzauber,
            vampiric_gifts,
            visions,
            wand_enchantments,
            weapon_enchantments,
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

    pub fn get_animal_disease(&self, id: u32) -> Option<&AnimalDisease> {
        self.animal_diseases.get(&id)
    }

    pub fn get_animal_shape(&self, id: u32) -> Option<&AnimalShape> {
        self.animal_shapes.get(&id)
    }

    pub fn get_animal_shape_path(&self, id: u32) -> Option<&AnimalShapePath> {
        self.animal_shape_paths.get(&id)
    }

    pub fn get_animal_shape_size(&self, id: u32) -> Option<&AnimalShapeSize> {
        self.animal_shape_sizes.get(&id)
    }

    pub fn get_animal_type(&self, id: u32) -> Option<&AnimalType> {
        self.animal_types.get(&id)
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

    pub fn get_arcane_orb_enchantment(&self, id: u32)
            -> Option<&ArcaneOrbEnchantment> {
        self.arcane_orb_enchantments.get(&id)
    }

    pub fn get_armor_type(&self, id: u32) -> Option<&ArmorType> {
        self.armor_types.get(&id)
    }

    pub fn get_aspect(&self, id: u32) -> Option<&Aspect> {
        self.aspects.get(&id)
    }

    pub fn get_attire_enchantment(&self, id: u32)
            -> Option<&AttireEnchantment> {
        self.attire_enchantments.get(&id)
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

    pub fn get_bowl_enchantment(&self, id: u32) -> Option<&BowlEnchantment> {
        self.bowl_enchantments.get(&id)
    }

    pub fn get_brawling_special_ability(&self, id: u32)
            -> Option<&BrawlingSpecialAbility> {
        self.brawling_special_abilities.get(&id)
    }

    pub fn get_brew(&self, id: u32) -> Option<&Brew> {
        self.brews.get(&id)
    }

    pub fn get_cantrip(&self, id: u32) -> Option<&Cantrip> {
        self.cantrips.get(&id)
    }

    pub fn get_cauldron_enchantment(&self, id: u32)
            -> Option<&CauldronEnchantment> {
        self.cauldron_enchantments.get(&id)
    }

    pub fn get_ceremonial_item_special_ability(&self, id: u32)
            -> Option<&CeremonialItemSpecialAbility> {
        self.ceremonial_item_special_abilities.get(&id)
    }

    pub fn get_ceremony(&self, id: u32) -> Option<&Ceremony> {
        self.ceremonies.get(&id)
    }

    pub fn get_chronicle_enchantment(&self, id: u32)
            -> Option<&ChronicleEnchantment> {
        self.chronicle_enchantments.get(&id)
    }

    pub fn get_combat_special_ability(&self, id: u32)
            -> Option<&CombatSpecialAbility> {
        self.combat_special_abilities.get(&id)
    }

    pub fn get_combat_special_ability_group(&self, id: u32)
            -> Option<&CombatSpecialAbilityGroup> {
        self.combat_special_ability_groups.get(&id)
    }

    pub fn get_combat_style_special_ability(&self, id: u32)
            -> Option<&CombatStyleSpecialAbility> {
        self.combat_style_special_abilities.get(&id)
    }

    pub fn get_combat_technique_group(&self, id: u32)
            -> Option<&CombatTechniqueGroup> {
        self.combat_technique_groups.get(&id)
    }

    pub fn get_command_special_ability(&self, id: u32)
            -> Option<&CommandSpecialAbility> {
        self.command_special_abilities.get(&id)
    }

    pub fn get_condition(&self, id: u32) -> Option<&Condition> {
        self.conditions.get(&id)
    }

    pub fn get_culture(&self, id: u32) -> Option<&Culture> {
        self.cultures.get(&id)
    }

    pub fn get_curriculum(&self, id: u32) -> Option<&Curriculum> {
        self.curricula.get(&id)
    }

    pub fn get_curse(&self, id: u32) -> Option<&Curse> {
        self.curses.get(&id)
    }

    pub fn get_dagger_ritual(&self, id: u32) -> Option<&DaggerRitual> {
        self.dagger_rituals.get(&id)
    }

    pub fn get_derived_characteristic(&self, id: u32)
            -> Option<&DerivedCharacteristic> {
        self.derived_characteristics.get(&id)
    }

    pub fn get_disadvantage(&self, id: u32) -> Option<&Disadvantage> {
        self.disadvantages.get(&id)
    }

    pub fn get_disease(&self, id: u32) -> Option<&Disease> {
        self.diseases.get(&id)
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

    pub fn get_equipment_package(&self, id: u32) -> Option<&EquipmentPackage> {
        self.equipment_packages.get(&id)
    }

    pub fn get_experience_level(&self, id: u32) -> Option<&ExperienceLevel> {
        self.experience_levels.get(&id)
    }

    pub fn get_eye_color(&self, id: u32) -> Option<&EyeColor> {
        self.eye_colors.get(&id)
    }

    pub fn get_familiar_special_ability(&self, id: u32)
            -> Option<&FamiliarSpecialAbility> {
        self.familiar_special_abilities.get(&id)
    }

    pub fn get_familiars_trick(&self, id: u32) -> Option<&FamiliarsTrick> {
        self.familiars_tricks.get(&id)
    }

    pub fn get_fate_point_sex_special_ability(&self, id: u32)
            -> Option<&FatePointSexSpecialAbility> {
        self.fate_point_sex_special_abilities.get(&id)
    }

    pub fn get_fate_point_special_ability(&self, id: u32)
            -> Option<&FatePointSpecialAbility> {
        self.fate_point_special_abilities.get(&id)
    }

    pub fn get_focus_rule(&self, id: u32) -> Option<&FocusRule> {
        self.focus_rules.get(&id)
    }

    pub fn get_fools_hat_enchantment(&self, id: u32)
            -> Option<&FoolsHatEnchantment> {
        self.fools_hat_enchantments.get(&id)
    }

    pub fn get_general_special_ability(&self, id: u32)
            -> Option<&GeneralSpecialAbility> {
        self.general_special_abilities.get(&id)
    }

    pub fn get_geode_ritual(&self, id: u32) -> Option<&GeodeRitual> {
        self.geode_rituals.get(&id)
    }

    pub fn get_guideline(&self, id: u32) -> Option<&Guideline> {
        self.guidelines.get(&id)
    }

    pub fn get_hair_color(&self, id: u32) -> Option<&HairColor> {
        self.hair_colors.get(&id)
    }

    pub fn get_influence(&self, id: u32) -> Option<&Influence> {
        self.influences.get(&id)
    }

    pub fn get_instrument_enchantment(&self, id: u32)
            -> Option<&InstrumentEnchantment> {
        self.instrument_enchantments.get(&id)
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

    pub fn get_krallenkettenzauber(&self, id: u32)
            -> Option<&Krallenkettenzauber> {
        self.krallenkettenzauber.get(&id)
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

    pub fn get_lycantropic_gift(&self, id: u32) -> Option<&LycantropicGift> {
        self.lycantropic_gifts.get(&id)
    }

    pub fn get_magical_dance(&self, id: u32) -> Option<&MagicalDance> {
        self.magical_dances.get(&id)
    }

    pub fn get_magical_melody(&self, id: u32) -> Option<&MagicalMelody> {
        self.magical_melodies.get(&id)
    }

    pub fn get_magical_rune(&self, id: u32) -> Option<&MagicalRune> {
        self.magical_runes.get(&id)
    }

    pub fn get_magical_special_ability(&self, id: u32)
            -> Option<&MagicalSpecialAbility> {
        self.magical_special_abilities.get(&id)
    }

    pub fn get_magical_tradition(&self, id: u32) -> Option<&MagicalTradition> {
        self.magical_traditions.get(&id)
    }

    pub fn get_magical_tradition_placeholder(&self, id: u32)
            -> Option<&MagicalTraditionPlaceholder> {
        self.magical_tradition_placeholders.get(&id)
    }

    pub fn get_magic_style_special_ability(&self, id: u32)
            -> Option<&MagicStyleSpecialAbility> {
        self.magic_style_special_abilities.get(&id)
    }

    pub fn get_melee_combat_technique(&self, id: u32)
            -> Option<&MeleeCombatTechnique> {
        self.melee_combat_techniques.get(&id)
    }

    pub fn get_optional_rule(&self, id: u32) -> Option<&OptionalRule> {
        self.optional_rules.get(&id)
    }

    pub fn get_orb_enchantment(&self, id: u32) -> Option<&OrbEnchantment> {
        self.orb_enchantments.get(&id)
    }

    pub fn get_pact_category(&self, id: u32) -> Option<&PactCategory> {
        self.pact_categories.get(&id)
    }

    pub fn get_pact_gift(&self, id: u32) -> Option<&PactGift> {
        self.pact_gifts.get(&id)
    }

    pub fn get_patron(&self, id: u32) -> Option<&Patron> {
        self.patrons.get(&id)
    }

    pub fn get_patron_categories(&self, id: u32) -> Option<&PatronCategory> {
        self.patron_categories.get(&id)
    }

    pub fn get_personality_trait(&self, id: u32) -> Option<&PersonalityTrait> {
        self.personality_traits.get(&id)
    }

    pub fn get_poison(&self, id: u32) -> Option<&Poison> {
        self.poisons.get(&id)
    }

    pub fn get_profession(&self, id: u32) -> Option<&Profession> {
        self.professions.get(&id)
    }

    pub fn get_property(&self, id: u32) -> Option<&Property> {
        self.properties.get(&id)
    }

    pub fn get_protective_warding_circle_special_ability(&self, id: u32)
            -> Option<&ProtectiveWardingCircleSpecialAbility> {
        self.protective_warding_circle_special_abilities.get(&id)
    }

    pub fn get_publication(&self, id: u32) -> Option<&Publication> {
        self.publications.get(&id)
    }

    pub fn get_race(&self, id: u32) -> Option<&Race> {
        self.races.get(&id)
    }

    pub fn get_ranged_combat_technique(&self, id: u32)
            -> Option<&RangedCombatTechnique> {
        self.ranged_combat_techniques.get(&id)
    }

    pub fn get_reach(&self, id: u32) -> Option<&Reach> {
        self.reaches.get(&id)
    }

    pub fn get_region(&self, id: u32) -> Option<&Region> {
        self.regions.get(&id)
    }

    pub fn get_ring_enchantment(&self, id: u32) -> Option<&RingEnchantment> {
        self.ring_enchantments.get(&id)
    }

    pub fn get_ritual(&self, id: u32) -> Option<&Ritual> {
        self.rituals.get(&id)
    }

    pub fn get_script(&self, id: u32) -> Option<&Script> {
        self.scripts.get(&id)
    }

    pub fn get_sermon(&self, id: u32) -> Option<&Sermon> {
        self.sermons.get(&id)
    }

    pub fn get_service(&self, id: u32) -> Option<&Service> {
        self.services.get(&id)
    }

    pub fn get_sex_practice(&self, id: u32) -> Option<&SexPractice> {
        self.sex_practices.get(&id)
    }

    pub fn get_sex_special_ability(&self, id: u32)
            -> Option<&SexSpecialAbility> {
        self.sex_special_abilities.get(&id)
    }

    pub fn get_sickle_ritual(&self, id: u32) -> Option<&SickleRitual> {
        self.sickle_rituals.get(&id)
    }

    pub fn get_sikaryan_drain_special_ability(&self, id: u32)
            -> Option<&SikaryanDrainSpecialAbility> {
        self.sikaryan_drain_special_abilities.get(&id)
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

    pub fn get_special_ability_group(&self, id: u32)
            -> Option<&SpecialAbilityGroup> {
        self.special_ability_groups.get(&id)
    }

    pub fn get_spell(&self, id: u32) -> Option<&Spell> {
        self.spells.get(&id)
    }

    pub fn get_spell_group(&self, id: u32) -> Option<&SpellGroup> {
        self.spell_groups.get(&id)
    }

    pub fn get_spell_sword_enchantment(&self, id: u32)
            -> Option<&SpellSwordEnchantment> {
        self.spell_sword_enchantments.get(&id)
    }

    pub fn get_staff_enchantment(&self, id: u32) -> Option<&StaffEnchantment> {
        self.staff_enchantments.get(&id)
    }

    pub fn get_state(&self, id: u32) -> Option<&State> {
        self.states.get(&id)
    }

    pub fn get_subject(&self, id: u32) -> Option<&Subject> {
        self.subjects.get(&id)
    }

    pub fn get_toy_enchantment(&self, id: u32) -> Option<&ToyEnchantment> {
        self.toy_enchantments.get(&id)
    }

    pub fn get_trade_secret(&self, id: u32) -> Option<&TradeSecret> {
        self.trade_secrets.get(&id)
    }

    pub fn get_tribe(&self, id: u32) -> Option<&Tribe> {
        self.tribes.get(&id)
    }

    pub fn get_trinkhornzauber(&self, id: u32) -> Option<&Trinkhornzauber> {
        self.trinkhornzauber.get(&id)
    }

    pub fn get_vampiric_gift(&self, id: u32) -> Option<&VampiricGift> {
        self.vampiric_gifts.get(&id)
    }

    pub fn get_vision(&self, id: u32) -> Option<&Vision> {
        self.visions.get(&id)
    }

    pub fn get_wand_enchantment(&self, id: u32) -> Option<&WandEnchantment> {
        self.wand_enchantments.get(&id)
    }

    pub fn get_weapon_enchantment(&self, id: u32)
            -> Option<&WeaponEnchantment> {
        self.weapon_enchantments.get(&id)
    }

    pub fn get_zibilja_ritual(&self, id: u32) -> Option<&ZibiljaRitual> {
        self.zibilja_rituals.get(&id)
    }

    pub fn get_ui_string(&self, locale: &str, id: &str) -> Option<&String> {
        self.uis.get(locale)
            .map(|ui| ui.get(id)
                .map(|v|
                    if let UIValue::Simple(s) = v { Some(s) } else { None })
                .flatten())
            .flatten()
    }

    pub fn get_as_entity(&self, id: Id) -> Option<Entity> {
        let int_id = id.internal_id();

        match id.category() {
            Category::AdvancedCombatSpecialAbilities =>
                to_entity(self.get_advanced_combat_special_ability(int_id)),
            Category::AdvancedKarmaSpecialAbilities =>
                to_entity(self.get_advanced_karma_special_ability(int_id)),
            Category::AdvancedMagicalSpecialAbilities =>
                to_entity(self.get_advanced_magical_special_ability(int_id)),
            Category::AdvancedSkillSpecialAbilities =>
                to_entity(self.get_advanced_skill_special_ability(int_id)),
            Category::Advantages =>
                to_entity(self.get_advantage(int_id)),
            Category::AncestorGlyphs =>
                to_entity(self.get_ancestor_glyph(int_id)),
            Category::AnimalDiseases =>
                to_entity(self.get_animal_disease(int_id)),
            Category::AnimalShapes =>
                to_entity(self.get_animal_shape(int_id)),
            Category::AnimalShapePaths =>
                to_entity(self.get_animal_shape_path(int_id)),
            Category::AnimalShapeSizes =>
                to_entity(self.get_animal_shape_size(int_id)),
            Category::AnimalTypes =>
                to_entity(self.get_animal_type(int_id)),
            Category::AnimistPowers =>
                to_entity(self.get_animist_power(int_id)),
            Category::ArcaneBardTraditions =>
                to_entity(self.get_arcane_bard_tradition(int_id)),
            Category::ArcaneDancerTraditions =>
                to_entity(self.get_arcane_dancer_tradition(int_id)),
            Category::ArcaneOrbEnchantments =>
                to_entity(self.get_arcane_orb_enchantment(int_id)),
            Category::ArmorTypes =>
                to_entity(self.get_armor_type(int_id)),
            Category::Aspects =>
                to_entity(self.get_aspect(int_id)),
            Category::AttireEnchantments =>
                to_entity(self.get_attire_enchantment(int_id)),
            Category::Attributes =>
                to_entity(self.get_attribute(int_id)),
            Category::BlessedTraditions =>
                to_entity(self.get_blessed_tradition(int_id)),
            Category::Blessings =>
                to_entity(self.get_blessing(int_id)),
            Category::BowlEnchantments =>
                to_entity(self.get_bowl_enchantment(int_id)),
            Category::BrawlingSpecialAbilities =>
                to_entity(self.get_brawling_special_ability(int_id)),
            Category::Brews =>
                to_entity(self.get_brew(int_id)),
            Category::Cantrips =>
                to_entity(self.get_cantrip(int_id)),
            Category::CauldronEnchantments =>
                to_entity(self.get_cauldron_enchantment(int_id)),
            Category::CeremonialItemSpecialAbilities =>
                to_entity(self.get_ceremonial_item_special_ability(int_id)),
            Category::Ceremonies =>
                to_entity(self.get_ceremony(int_id)),
            Category::ChronicleEnchantments =>
                to_entity(self.get_chronicle_enchantment(int_id)),
            Category::CombatSpecialAbilities =>
                to_entity(self.get_combat_special_ability(int_id)),
            Category::CombatSpecialAbilityGroups =>
                to_entity(self.get_combat_special_ability_group(int_id)),
            Category::CombatStyleSpecialAbilities =>
                to_entity(self.get_combat_style_special_ability(int_id)),
            Category::CombatTechniqueGroups =>
                to_entity(self.get_combat_technique_group(int_id)),
            Category::CommandSpecialAbilities =>
                to_entity(self.get_command_special_ability(int_id)),
            Category::Conditions =>
                to_entity(self.get_condition(int_id)),
            Category::Cultures =>
                to_entity(self.get_culture(int_id)),
            Category::Curricula =>
                to_entity(self.get_curriculum(int_id)),
            Category::Curses =>
                to_entity(self.get_curse(int_id)),
            Category::DaggerRituals =>
                to_entity(self.get_dagger_ritual(int_id)),
            Category::DerivedCharacteristics =>
                to_entity(self.get_derived_characteristic(int_id)),
            Category::Disadvantages =>
                to_entity(self.get_disadvantage(int_id)),
            Category::Diseases =>
                to_entity(self.get_disease(int_id)),
            Category::DominationRituals =>
                to_entity(self.get_domination_ritual(int_id)),
            Category::Elements =>
                to_entity(self.get_element(int_id)),
            Category::ElvenMagicalSongs =>
                to_entity(self.get_elven_magical_song(int_id)),
            Category::EquipmentPackages =>
                to_entity(self.get_equipment_package(int_id)),
            Category::ExperienceLevels =>
                to_entity(self.get_experience_level(int_id)),
            Category::EyeColors =>
                to_entity(self.get_eye_color(int_id)),
            Category::FamiliarSpecialAbilities =>
                to_entity(self.get_familiar_special_ability(int_id)),
            Category::FamiliarsTricks =>
                to_entity(self.get_familiars_trick(int_id)),
            Category::FatePointSexSpecialAbilities =>
                to_entity(self.get_fate_point_sex_special_ability(int_id)),
            Category::FatePointSpecialAbilities =>
                to_entity(self.get_fate_point_special_ability(int_id)),
            Category::FocusRules =>
                to_entity(self.get_focus_rule(int_id)),
            Category::FoolsHatEnchantments =>
                to_entity(self.get_fools_hat_enchantment(int_id)),
            Category::GeneralSpecialAbilities =>
                to_entity(self.get_general_special_ability(int_id)),
            Category::GeodeRituals =>
                to_entity(self.get_geode_ritual(int_id)),
            Category::Guidelines =>
                to_entity(self.get_guideline(int_id)),
            Category::HairColors =>
                to_entity(self.get_hair_color(int_id)),
            Category::Influences =>
                to_entity(self.get_influence(int_id)),
            Category::InstrumentEnchantments =>
                to_entity(self.get_instrument_enchantment(int_id)),
            Category::ItemGroups =>
                to_entity(self.get_item_group(int_id)),
            Category::JesterTricks =>
                to_entity(self.get_jester_trick(int_id)),
            Category::KarmaSpecialAbilities =>
                to_entity(self.get_karma_special_ability(int_id)),
            Category::Krallenkettenzauber =>
                to_entity(self.get_krallenkettenzauber(int_id)),
            Category::Languages =>
                to_entity(self.get_language(int_id)),
            Category::LiturgicalChants =>
                to_entity(self.get_liturgical_chant(int_id)),
            Category::LiturgicalChantGroups =>
                to_entity(self.get_liturgical_chant_group(int_id)),
            Category::LiturgicalStyleSpecialAbilities =>
                to_entity(self.get_liturgical_style_special_ability(int_id)),
            Category::LycantropicGifts =>
                to_entity(self.get_lycantropic_gift(int_id)),
            Category::MagicalDances =>
                to_entity(self.get_magical_dance(int_id)),
            Category::MagicalMelodies =>
                to_entity(self.get_magical_melody(int_id)),
            Category::MagicalRunes =>
                to_entity(self.get_magical_rune(int_id)),
            Category::MagicalSpecialAbilities =>
                to_entity(self.get_magical_special_ability(int_id)),
            Category::MagicalTraditions =>
                to_entity(self.get_magical_tradition(int_id)),
            Category::MagicalTraditionPlaceholders =>
                to_entity(self.get_magical_tradition_placeholder(int_id)),
            Category::MagicStyleSpecialAbilities =>
                to_entity(self.get_magic_style_special_ability(int_id)),
            Category::MeleeCombatTechniques =>
                to_entity(self.get_melee_combat_technique(int_id)),
            Category::OptionalRules =>
                to_entity(self.get_optional_rule(int_id)),
            Category::OrbEnchantments =>
                to_entity(self.get_orb_enchantment(int_id)),
            Category::PactCategories =>
                to_entity(self.get_pact_category(int_id)),
            Category::PactGifts =>
                to_entity(self.get_pact_gift(int_id)),
            Category::Patrons =>
                to_entity(self.get_patron(int_id)),
            Category::PatronCategories =>
                to_entity(self.get_patron_categories(int_id)),
            Category::PersonalityTraits =>
                to_entity(self.get_personality_trait(int_id)),
            Category::Poisons =>
                to_entity(self.get_poison(int_id)),
            Category::Professions =>
                to_entity(self.get_profession(int_id)),
            Category::Properties =>
                to_entity(self.get_property(int_id)),
            Category::ProtectiveWardingCircleSpecialAbilities =>
                to_entity(self.get_protective_warding_circle_special_ability(
                    int_id)),
            Category::Publications =>
                to_entity(self.get_publication(int_id)),
            Category::Races =>
                to_entity(self.get_race(int_id)),
            Category::RangedCombatTechniques =>
                to_entity(self.get_ranged_combat_technique(int_id)),
            Category::Reaches =>
                to_entity(self.get_reach(int_id)),
            Category::Regions =>
                to_entity(self.get_region(int_id)),
            Category::RingEnchantments =>
                to_entity(self.get_ring_enchantment(int_id)),
            Category::Rituals =>
                to_entity(self.get_ritual(int_id)),
            Category::Scripts =>
                to_entity(self.get_script(int_id)),
            Category::Sermons =>
                to_entity(self.get_sermon(int_id)),
            Category::Services =>
                to_entity(self.get_service(int_id)),
            Category::SexPractices =>
                to_entity(self.get_sex_practice(int_id)),
            Category::SexSpecialAbilities =>
                to_entity(self.get_sex_special_ability(int_id)),
            Category::SickleRituals =>
                to_entity(self.get_sickle_ritual(int_id)),
            Category::SikaryanDrainSpecialAbilities =>
                to_entity(self.get_sikaryan_drain_special_ability(int_id)),
            Category::Skills =>
                to_entity(self.get_skill(int_id)),
            Category::SkillGroups =>
                to_entity(self.get_skill_group(int_id)),
            Category::SkillStyleSpecialAbilities =>
                to_entity(self.get_skill_style_special_ability(int_id)),
            Category::SocialStatuses =>
                to_entity(self.get_social_status(int_id)),
            Category::SpecialAbilityGroups =>
                to_entity(self.get_special_ability_group(int_id)),
            Category::Spells =>
                to_entity(self.get_spell(int_id)),
            Category::SpellGroups =>
                to_entity(self.get_spell_group(int_id)),
            Category::SpellSwordEnchantments =>
                to_entity(self.get_spell_sword_enchantment(int_id)),
            Category::StaffEnchantments =>
                to_entity(self.get_staff_enchantment(int_id)),
            Category::States =>
                to_entity(self.get_state(int_id)),
            Category::Subjects =>
                to_entity(self.get_subject(int_id)),
            Category::ToyEnchantments =>
                to_entity(self.get_toy_enchantment(int_id)),
            Category::TradeSecrets =>
                to_entity(self.get_trade_secret(int_id)),
            Category::Tribes =>
                to_entity(self.get_tribe(int_id)),
            Category::Trinkhornzauber =>
                to_entity(self.get_trinkhornzauber(int_id)),
            Category::VampiricGifts =>
                to_entity(self.get_vampiric_gift(int_id)),
            Category::Visions =>
                to_entity(self.get_vision(int_id)),
            Category::WandEnchantments =>
                to_entity(self.get_wand_enchantment(int_id)),
            Category::WeaponEnchantments =>
                to_entity(self.get_weapon_enchantment(int_id)),
            Category::ZibiljaRituals =>
                to_entity(self.get_zibilja_ritual(int_id)),
            _ => None // TODO update until all are implemented
        }
    }
}

fn to_entity<'a, T>(opt_t: Option<&'a T>) -> Option<Entity>
where
    Entity: From<&'a T>,
    T: 'a
{
    Some(Entity::from(opt_t?))
}

/// A trait for localizations for entities. The minimal requirement for each
/// localization is to provide the name of its entity.
pub trait Localization {
    fn name(&self) -> &str;

    fn name_as_select_option(&self) -> &str {
        self.name()
    }
}

impl Localization for String {
    fn name(&self) -> &str {
        &self
    }
}

impl<L: Localization> Localization for &L {
    fn name(&self) -> &str {
        (*self).name()
    }

    fn name_as_select_option(&self) -> &str {
        (*self).name_as_select_option()
    }
}

/// A map of language identifiers to [Localization]s of type `L`.
pub type Translations<L> = HashMap<String, L>;

/// A trait for entities which are translatable, i.e. for which translations
/// of some [Localization] type exist.
pub trait Translatable<'a, L: Localization> {
    fn translate(&'a self, locale: &str) -> Option<L>;
}

/// A trait for entities which are translatable by providing [Translations] of
/// some [Localization] type. The [Translatable] trait is implemented by
/// blanket-implementation.
pub trait TranslationsTranslatable {
    type Localization: Localization;

    fn translations(&self) -> &Translations<Self::Localization>;
}

impl<'a, T, L> Translatable<'a, &'a L> for T
where
    L: Localization,
    T: TranslationsTranslatable<Localization = L>
{
    fn translate(&'a self, locale: &str) -> Option<&'a L> {
        self.translations().get(locale)
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SingleOrList<T> {
    Single(T),
    List(Vec<T>)
}

pub type Ids = SingleOrList<u32>;

#[derive(Clone, Deserialize, Serialize)]
pub struct SuggestedUnsuitable {
    pub id: u32
}
