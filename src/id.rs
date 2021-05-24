use serde::{Deserialize, Serialize};

pub const PREFIX_ADV: &str = "ADV";
pub const PREFIX_DISADV: &str = "DISADV";
pub const PREFIX_SA: &str = "SA";

/// An enumeration of all categories of data that may be referenced.
#[derive(Copy, Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Category {
    AdvancedCombatSpecialAbilities,
    AdvancedKarmaSpecialAbilities,
    AdvancedMagicalSpecialAbilities,
    AdvancedSkillSpecialAbilities,
    Advantages,
    AncestorGlyphs,
    AnimalDiseases,
    AnimalShapePaths,
    AnimalShapes,
    AnimalShapeSizes,
    AnimalTypes,
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
    FamiliarsTricks,
    FatePointSexSpecialAbilities,
    FatePointSpecialAbilities,
    FocusRules,
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
    OptionalRules,
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
    Regions,
    RingEnchantments,
    Rituals,
    Scripts,
    Sermons,
    Services,
    SexPractices,
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
