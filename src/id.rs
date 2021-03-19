use serde::{Deserialize, Serialize};

pub const PREFIX_ADV: &str = "ADV";
pub const PREFIX_DISADV: &str = "DISADV";
pub const PREFIX_SA: &str = "SA";

pub const ADV_NIMBLE: u32 = 9;
pub const ADV_BLESSED: u32 = 12;
pub const ADV_INCREASED_ASTRAL_POWER: u32 = 20;
pub const ADV_INCREASED_KARMAL_POINTS: u32 = 21;
pub const ADV_INCREASED_LIFE_POINTS: u32 = 22;
pub const ADV_INCREASED_SPIRIT: u32 = 23;
pub const ADV_INCREASED_TOUGHNESS: u32 = 24;
pub const ADV_SPELLCASTER: u32 = 47;

pub const ATTR_COURAGE: u32 = 1;
pub const ATTR_SAGACITY: u32 = 2;
pub const ATTR_INTUITION: u32 = 3;
pub const ATTR_CHARISMA: u32 = 4;
pub const ATTR_DEXTERITY: u32 = 5;
pub const ATTR_AGILITY: u32 = 6;
pub const ATTR_CONSTITUTION: u32 = 7;
pub const ATTR_STRENGTH: u32 = 8;

pub const COND_PAIN: u32 = 6;

pub const DC_LIFE_POINTS: u32 = 1;
pub const DC_ASTRAL_POINTS: u32 = 2;
pub const DC_KARMAL_POINTS: u32 = 3;
pub const DC_SPIRIT: u32 = 4;
pub const DC_TOUGHNESS: u32 = 5;
pub const DC_DODGE: u32 = 6;
pub const DC_INITIATIVE: u32 = 7;
pub const DC_MOVEMENT: u32 = 8;
pub const DC_WOUND_THRESHOLD: u32 = 9;

pub const DISADV_SLOW: u32 = 4;
pub const DISADV_DECREASED_ARCANE_POWER: u32 = 23;
pub const DISADV_DECREASED_KARMA_POINTS: u32 = 24;
pub const DISADV_DECREASED_LIFE_POINTS: u32 = 25;
pub const DISADV_DECREASED_SPIRIT: u32 = 26;
pub const DISADV_DECREASED_TOUGHNESS: u32 = 27;
pub const DISADV_MAIMED: u32 = 48;
pub const DISADV_MAIMED_ONE_LEGGED: u32 = 3;

pub const L10N_ADVANTAGES: &str = "header.tabs.advantages";
pub const L10N_DISADVANTAGES: &str = "header.tabs.disadvantages";
pub const L10N_SKILLS: &str = "header.tabs.skills";

/// An enumeration of all categories of data that may be referenced.
#[derive(Copy, Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Category {
    AdvancedCombatSpecialAbilities,
    AdvancedKarmaSpecialAbilities,
    AdvancedMagicalSpecialAbilities,
    AdvancedSkillSpecialAbilities,
    Advantages,
    AncestorGlyphs,
    AnimalShapePaths,
    AnimalShapes,
    AnimalShapeSizes,
    AnimistPowers,
    ArcaneBardTraditions,
    ArcaneDancerTraditions,
    ArcaneOrbEnchantments,
    Armors,
    ArmorTypes,
    Aspects,
    AttireEnchantments,
    Attributes,
    BlessedTraditions,
    Blessings,
    BowlEnchantments,
    BrawlingSpecialAbilities,
    Brews,
    Cantrips,
    CauldronEnchantments,
    CeremonialItemSpecialAbilities,
    Ceremonies,
    ChronicleEnchantments,
    CombatSpecialAbilities,
    CombatSpecialAbilityGroups,
    CombatStyleSpecialAbilities,
    CombatTechniqueGroups,
    CommandSpecialAbilities,
    Conditions,
    Cultures,
    Curricula,
    Curses,
    DaggerRituals,
    DerivedCharacteristics,
    Disadvantages,
    Diseases,
    DominationRituals,
    Elements,
    ElvenMagicalSongs,
    EquipmentPackages,
    ExperienceLevels,
    EyeColors,
    FamiliarSpecialAbilities,
    FatePointSexSpecialAbilities,
    FatePointSpecialAbilities,
    FoolsHatEnchantments,
    GeneralSpecialAbilities,
    GeodeRituals,
    Guidelines,
    HairColors,
    Influences,
    InstrumentEnchantments,
    ItemGroups,
    JesterTricks,
    KarmaSpecialAbilities,
    Krallenkettenzauber,
    Languages,
    LiturgicalChantGroups,
    LiturgicalChants,
    LiturgicalStyleSpecialAbilities,
    LycantropicGifts,
    MagicalDances,
    MagicalMelodies,
    MagicalRunes,
    MagicalSpecialAbilities,
    MagicalTraditions,
    MagicalTraditionPlaceholders,
    MagicStyleSpecialAbilities,
    MeleeCombatTechniques,
    OrbEnchantments,
    PactCategories,
    PactGifts,
    PatronCategories,
    Patrons,
    PersonalityTraits,
    Poisons,
    Professions,
    Properties,
    ProtectiveWardingCircleSpecialAbilities,
    Publications,
    Races,
    RangedCombatTechniques,
    Reaches,
    RingEnchantments,
    Rituals,
    Scripts,
    Sermons,
    SexSpecialAbilities,
    SickleRituals,
    SikaryanDrainSpecialAbilities,
    Skills,
    SkillGroups,
    SkillStyleSpecialAbilities,
    SocialStatuses,
    SpecialAbilityGroups,
    SpellGroups,
    Spells,
    SpellSwordEnchantments,
    StaffEnchantments,
    States,
    Subjects,
    ToyEnchantments,
    TradeSecrets,
    Tribes,
    Trinkhornzauber,
    VampiricGifts,
    Visions,
    WandEnchantments,
    WeaponEnchantments,
    ZibiljaRituals
}

/// A universal ID for data entries.
#[derive(Copy, Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Id {
    category: Category,
    internal_id: u32
}

impl Id {

    /// Creates a new ID from the [Category] and internal ID.
    pub const fn new(category: Category, internal_id: u32) -> Id {
        Id {
            category,
            internal_id
        }
    }

    /// The category of the data entry identified by this ID.
    pub fn category(&self) -> Category {
        self.category
    }

    /// The internal ID of this data entry within its category.
    pub fn internal_id(&self) -> u32 {
        self.internal_id
    }
}

/// A trait for all Optolith data structs which have a numeric ID.
pub trait Identifiable {
    fn id(&self) -> Id;

    fn internal_id(&self) -> u32 {
        self.id().internal_id()
    }
}

// TODO replace with const generics once available

/// A replacement for a const generic [Category], which specifies a single
/// category.
pub trait CategoryProvider {
    const CATEGORY: Category;
}

/// Gets a string-ID's prefix.
pub fn get_prefix(s: &str) -> &str {
    s.split('_').next().unwrap()
}

/// Converts a string-ID parsed from a character file to a numeric ID.
pub fn to_id(s: &str) -> u32 {
    s.split('_').last().unwrap().parse().unwrap()
}
