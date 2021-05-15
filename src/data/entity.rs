use crate::data::{Localization, Translatable};
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
use crate::data::status_effect::{
    AnimalDisease,
    Condition,
    Disease,
    Poison,
    State
};
use crate::data::derived_characteristic::DerivedCharacteristic;
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
use crate::id::{Id, Identifiable};

/// A wrapper for any entity that is part of the Optolith database.
#[derive(Clone)]
pub enum Entity {
    AdvancedCombatSpecialAbility(AdvancedCombatSpecialAbility),
    AdvancedKarmaSpecialAbility(AdvancedKarmaSpecialAbility),
    AdvancedMagicalSpecialAbility(AdvancedMagicalSpecialAbility),
    AdvancedSkillSpecialAbility(AdvancedSkillSpecialAbility),
    Advantage(Advantage),
    AncestorGlyph(AncestorGlyph),
    AnimalDisease(AnimalDisease),
    AnimalShape(AnimalShape),
    AnimalShapePath(AnimalShapePath),
    AnimalShapeSize(AnimalShapeSize),
    AnimalType(AnimalType),
    AnimistPower(AnimistPower),
    ArcaneBardTradition(ArcaneBardTradition),
    ArcaneDancerTradition(ArcaneDancerTradition),
    ArcaneOrbEnchantment(ArcaneOrbEnchantment),
    ArmorType(ArmorType),
    Aspect(Aspect),
    AttireEnchantment(AttireEnchantment),
    Attribute(Attribute),
    BlessedTradition(BlessedTradition),
    Blessing(Blessing),
    BowlEnchantment(BowlEnchantment),
    BrawlingSpecialAbility(BrawlingSpecialAbility),
    Brew(Brew),
    Cantrip(Cantrip),
    CauldronEnchantment(CauldronEnchantment),
    CeremonialItemSpecialAbility(CeremonialItemSpecialAbility),
    Ceremony(Ceremony),
    ChronicleEnchantment(ChronicleEnchantment),
    CombatSpecialAbility(CombatSpecialAbility),
    CombatSpecialAbilityGroup(CombatSpecialAbilityGroup),
    CombatStyleSpecialAbility(CombatStyleSpecialAbility),
    CombatTechniqueGroup(CombatTechniqueGroup),
    CommandSpecialAbility(CommandSpecialAbility),
    Condition(Condition),
    Culture(Culture),
    Curriculum(Curriculum),
    Curse(Curse),
    DaggerRitual(DaggerRitual),
    DerivedCharacteristic(DerivedCharacteristic),
    Disadvantage(Disadvantage),
    Disease(Disease),
    DominationRitual(DominationRitual),
    Element(Element),
    ElvenMagicalSong(ElvenMagicalSong),
    EquipmentPackage(EquipmentPackage),
    ExperienceLevel(ExperienceLevel),
    EyeColor(EyeColor),
    FamiliarSpecialAbility(FamiliarSpecialAbility),
    FamiliarsTrick(FamiliarsTrick),
    FatePointSexSpecialAbility(FatePointSexSpecialAbility),
    FatePointSpecialAbility(FatePointSpecialAbility),
    FocusRule(FocusRule),
    FoolsHatEnchantment(FoolsHatEnchantment),
    GeneralSpecialAbility(GeneralSpecialAbility),
    GeodeRitual(GeodeRitual),
    Guideline(Guideline),
    HairColor(HairColor),
    Influence(Influence),
    InstrumentEnchantment(InstrumentEnchantment),
    ItemGroup(ItemGroup),
    JesterTrick(JesterTrick),
    KarmaSpecialAbility(KarmaSpecialAbility),
    Krallenkettenzauber(Krallenkettenzauber),
    Language(Language),
    LiturgicalChant(LiturgicalChant),
    LiturgicalChantGroup(LiturgicalChantGroup),
    LiturgicalStyleSpecialAbility(LiturgicalStyleSpecialAbility),
    LycantropicGift(LycantropicGift),
    MagicalDance(MagicalDance),
    MagicalMelody(MagicalMelody),
    MagicalRune(MagicalRune),
    MagicalSpecialAbility(MagicalSpecialAbility),
    MagicalTradition(MagicalTradition),
    MagicalTraditionPlaceholder(MagicalTraditionPlaceholder),
    MagicStyleSpecialAbility(MagicStyleSpecialAbility),
    MeleeCombatTechnique(MeleeCombatTechnique),
    OptionalRule(OptionalRule),
    OrbEnchantment(OrbEnchantment),
    PactCategory(PactCategory),
    PactGift(PactGift),
    Patron(Patron),
    PatronCategory(PatronCategory),
    PersonalityTrait(PersonalityTrait),
    Poison(Poison),
    Profession(Profession),
    Property(Property),
    ProtectiveWardingCircleSpecialAbility(
        ProtectiveWardingCircleSpecialAbility),
    Publication(Publication),
    Race(Race),
    RangedCombatTechnique(RangedCombatTechnique),
    Reach(Reach),
    Region(Region),
    RingEnchantment(RingEnchantment),
    Ritual(Ritual),
    Script(Script),
    Sermon(Sermon),
    Service(Service),
    SexPractice(SexPractice),
    SexSpecialAbility(SexSpecialAbility),
    SickleRitual(SickleRitual),
    SikaryanDrainSpecialAbility(SikaryanDrainSpecialAbility),
    Skill(Skill),
    SkillGroup(SkillGroup),
    SkillStyleSpecialAbility(SkillStyleSpecialAbility),
    SocialStatus(SocialStatus),
    SpecialAbilityGroup(SpecialAbilityGroup),
    SpellGroup(SpellGroup),
    Spell(Spell),
    SpellSwordEnchantment(SpellSwordEnchantment),
    StaffEnchantment(StaffEnchantment),
    State(State),
    Subject(Subject),
    ToyEnchantment(ToyEnchantment),
    TradeSecret(TradeSecret),
    Tribe(Tribe),
    Trinkhornzauber(Trinkhornzauber),
    VampiricGift(VampiricGift),
    Vision(Vision),
    WandEnchantment(WandEnchantment),
    WeaponEnchantment(WeaponEnchantment),
    ZibiljaRitual(ZibiljaRitual)
}

/// A [Localization] that stores cloned values of the localization of any
/// entity. Only relevant information to implement the localization trait is
/// stored - the rest is discarded.
pub struct FlattenedLocalization {
    name: String,
    name_as_select_option: String
}

impl FlattenedLocalization {
    fn new(localization: &impl Localization) -> FlattenedLocalization {
        FlattenedLocalization {
            name: localization.name().to_owned(),
            name_as_select_option:
                localization.name_as_select_option().to_owned()
        }
    }
}

impl Localization for FlattenedLocalization {
    fn name(&self) -> &str {
        &self.name
    }

    fn name_as_select_option(&self) -> &str {
        &self.name_as_select_option
    }
}

impl Identifiable for Entity {
    fn id(&self) -> Id {
        match self {
            Entity::AdvancedCombatSpecialAbility(e) => e.id(),
            Entity::AdvancedKarmaSpecialAbility(e) => e.id(),
            Entity::AdvancedMagicalSpecialAbility(e) => e.id(),
            Entity::AdvancedSkillSpecialAbility(e) => e.id(),
            Entity::Advantage(e) => e.id(),
            Entity::AncestorGlyph(e) => e.id(),
            Entity::AnimalDisease(e) => e.id(),
            Entity::AnimalShape(e) => e.id(),
            Entity::AnimalShapePath(e) => e.id(),
            Entity::AnimalShapeSize(e) => e.id(),
            Entity::AnimalType(e) => e.id(),
            Entity::AnimistPower(e) => e.id(),
            Entity::ArcaneBardTradition(e) => e.id(),
            Entity::ArcaneDancerTradition(e) => e.id(),
            Entity::ArcaneOrbEnchantment(e) => e.id(),
            Entity::ArmorType(e) => e.id(),
            Entity::Aspect(e) => e.id(),
            Entity::AttireEnchantment(e) => e.id(),
            Entity::Attribute(e) => e.id(),
            Entity::BlessedTradition(e) => e.id(),
            Entity::Blessing(e) => e.id(),
            Entity::BowlEnchantment(e) => e.id(),
            Entity::BrawlingSpecialAbility(e) => e.id(),
            Entity::Brew(e) => e.id(),
            Entity::Cantrip(e) => e.id(),
            Entity::CauldronEnchantment(e) => e.id(),
            Entity::CeremonialItemSpecialAbility(e) => e.id(),
            Entity::Ceremony(e) => e.id(),
            Entity::ChronicleEnchantment(e) => e.id(),
            Entity::CombatSpecialAbility(e) => e.id(),
            Entity::CombatSpecialAbilityGroup(e) => e.id(),
            Entity::CombatStyleSpecialAbility(e) => e.id(),
            Entity::CombatTechniqueGroup(e) => e.id(),
            Entity::CommandSpecialAbility(e) => e.id(),
            Entity::Condition(e) => e.id(),
            Entity::Culture(e) => e.id(),
            Entity::Curriculum(e) => e.id(),
            Entity::Curse(e) => e.id(),
            Entity::DaggerRitual(e) => e.id(),
            Entity::DerivedCharacteristic(e) => e.id(),
            Entity::Disadvantage(e) => e.id(),
            Entity::Disease(e) => e.id(),
            Entity::DominationRitual(e) => e.id(),
            Entity::Element(e) => e.id(),
            Entity::ElvenMagicalSong(e) => e.id(),
            Entity::EquipmentPackage(e) => e.id(),
            Entity::ExperienceLevel(e) => e.id(),
            Entity::EyeColor(e) => e.id(),
            Entity::FamiliarSpecialAbility(e) => e.id(),
            Entity::FamiliarsTrick(e) => e.id(),
            Entity::FatePointSexSpecialAbility(e) => e.id(),
            Entity::FatePointSpecialAbility(e) => e.id(),
            Entity::FocusRule(e) => e.id(),
            Entity::FoolsHatEnchantment(e) => e.id(),
            Entity::GeneralSpecialAbility(e) => e.id(),
            Entity::GeodeRitual(e) => e.id(),
            Entity::Guideline(e) => e.id(),
            Entity::HairColor(e) => e.id(),
            Entity::Influence(e) => e.id(),
            Entity::InstrumentEnchantment(e) => e.id(),
            Entity::ItemGroup(e) => e.id(),
            Entity::JesterTrick(e) => e.id(),
            Entity::KarmaSpecialAbility(e) => e.id(),
            Entity::Krallenkettenzauber(e) => e.id(),
            Entity::Language(e) => e.id(),
            Entity::LiturgicalChant(e) => e.id(),
            Entity::LiturgicalChantGroup(e) => e.id(),
            Entity::LiturgicalStyleSpecialAbility(e) => e.id(),
            Entity::LycantropicGift(e) => e.id(),
            Entity::MagicalDance(e) => e.id(),
            Entity::MagicalMelody(e) => e.id(),
            Entity::MagicalRune(e) => e.id(),
            Entity::MagicalSpecialAbility(e) => e.id(),
            Entity::MagicalTradition(e) => e.id(),
            Entity::MagicalTraditionPlaceholder(e) => e.id(),
            Entity::MagicStyleSpecialAbility(e) => e.id(),
            Entity::MeleeCombatTechnique(e) => e.id(),
            Entity::OptionalRule(e) => e.id(),
            Entity::OrbEnchantment(e) => e.id(),
            Entity::PactCategory(e) => e.id(),
            Entity::PactGift(e) => e.id(),
            Entity::Patron(e) => e.id(),
            Entity::PatronCategory(e) => e.id(),
            Entity::PersonalityTrait(e) => e.id(),
            Entity::Poison(e) => e.id(),
            Entity::Profession(e) => e.id(),
            Entity::Property(e) => e.id(),
            Entity::ProtectiveWardingCircleSpecialAbility(e) => e.id(),
            Entity::Publication(e) => e.id(),
            Entity::Race(e) => e.id(),
            Entity::RangedCombatTechnique(e) => e.id(),
            Entity::Reach(e) => e.id(),
            Entity::Region(e) => e.id(),
            Entity::RingEnchantment(e) => e.id(),
            Entity::Ritual(e) => e.id(),
            Entity::Script(e) => e.id(),
            Entity::Sermon(e) => e.id(),
            Entity::Service(e) => e.id(),
            Entity::SexPractice(e) => e.id(),
            Entity::SexSpecialAbility(e) => e.id(),
            Entity::SickleRitual(e) => e.id(),
            Entity::SikaryanDrainSpecialAbility(e) => e.id(),
            Entity::Skill(e) => e.id(),
            Entity::SkillGroup(e) => e.id(),
            Entity::SkillStyleSpecialAbility(e) => e.id(),
            Entity::SocialStatus(e) => e.id(),
            Entity::SpecialAbilityGroup(e) => e.id(),
            Entity::SpellGroup(e) => e.id(),
            Entity::Spell(e) => e.id(),
            Entity::SpellSwordEnchantment(e) => e.id(),
            Entity::StaffEnchantment(e) => e.id(),
            Entity::State(e) => e.id(),
            Entity::Subject(e) => e.id(),
            Entity::ToyEnchantment(e) => e.id(),
            Entity::TradeSecret(e) => e.id(),
            Entity::Tribe(e) => e.id(),
            Entity::Trinkhornzauber(e) => e.id(),
            Entity::VampiricGift(e) => e.id(),
            Entity::Vision(e) => e.id(),
            Entity::WandEnchantment(e) => e.id(),
            Entity::WeaponEnchantment(e) => e.id(),
            Entity::ZibiljaRitual(e) => e.id()
        }
    }
}

fn translate_flat<'a, L>(t: &'a (impl Translatable<'a, &'a L> + 'a),
    locale: &str) -> Option<FlattenedLocalization>
where
    L: Localization + 'a
{
    Some(FlattenedLocalization::new(t.translate(locale)?))
}

impl<'a> Translatable<'a, FlattenedLocalization> for Entity {
    fn translate(&'a self, l: &str) -> Option<FlattenedLocalization> {
        match self {
            Entity::AdvancedCombatSpecialAbility(e) => translate_flat(e, l),
            Entity::AdvancedKarmaSpecialAbility(e) => translate_flat(e, l),
            Entity::AdvancedMagicalSpecialAbility(e) => translate_flat(e, l),
            Entity::AdvancedSkillSpecialAbility(e) => translate_flat(e, l),
            Entity::Advantage(e) => translate_flat(e, l),
            Entity::AncestorGlyph(e) => translate_flat(e, l),
            Entity::AnimalDisease(e) => translate_flat(e, l),
            Entity::AnimalShape(e) => translate_flat(e, l),
            Entity::AnimalShapePath(e) => translate_flat(e, l),
            Entity::AnimalShapeSize(e) => translate_flat(e, l),
            Entity::AnimalType(e) => translate_flat(e, l),
            Entity::AnimistPower(e) => translate_flat(e, l),
            Entity::ArcaneBardTradition(e) => translate_flat(e, l),
            Entity::ArcaneDancerTradition(e) => translate_flat(e, l),
            Entity::ArcaneOrbEnchantment(e) => translate_flat(e, l),
            Entity::ArmorType(e) => translate_flat(e, l),
            Entity::Aspect(e) => translate_flat(e, l),
            Entity::AttireEnchantment(e) => translate_flat(e, l),
            Entity::Attribute(e) => translate_flat(e, l),
            Entity::BlessedTradition(e) => translate_flat(e, l),
            Entity::Blessing(e) => translate_flat(e, l),
            Entity::BowlEnchantment(e) => translate_flat(e, l),
            Entity::BrawlingSpecialAbility(e) => translate_flat(e, l),
            Entity::Brew(e) => translate_flat(e, l),
            Entity::Cantrip(e) => translate_flat(e, l),
            Entity::CauldronEnchantment(e) => translate_flat(e, l),
            Entity::CeremonialItemSpecialAbility(e) => translate_flat(e, l),
            Entity::Ceremony(e) => translate_flat(e, l),
            Entity::ChronicleEnchantment(e) => translate_flat(e, l),
            Entity::CombatSpecialAbility(e) => translate_flat(e, l),
            Entity::CombatSpecialAbilityGroup(e) => translate_flat(e, l),
            Entity::CombatStyleSpecialAbility(e) => translate_flat(e, l),
            Entity::CombatTechniqueGroup(e) => translate_flat(e, l),
            Entity::CommandSpecialAbility(e) => translate_flat(e, l),
            Entity::Condition(e) => translate_flat(e, l),
            Entity::Culture(e) => translate_flat(e, l),
            Entity::Curriculum(e) => translate_flat(e, l),
            Entity::Curse(e) => translate_flat(e, l),
            Entity::DaggerRitual(e) => translate_flat(e, l),
            Entity::DerivedCharacteristic(e) => translate_flat(e, l),
            Entity::Disadvantage(e) => translate_flat(e, l),
            Entity::Disease(e) => translate_flat(e, l),
            Entity::DominationRitual(e) => translate_flat(e, l),
            Entity::Element(e) => translate_flat(e, l),
            Entity::ElvenMagicalSong(e) => translate_flat(e, l),
            Entity::EquipmentPackage(e) => translate_flat(e, l),
            Entity::ExperienceLevel(e) => translate_flat(e, l),
            Entity::EyeColor(e) => translate_flat(e, l),
            Entity::FamiliarSpecialAbility(e) => translate_flat(e, l),
            Entity::FamiliarsTrick(e) => translate_flat(e, l),
            Entity::FatePointSexSpecialAbility(e) => translate_flat(e, l),
            Entity::FatePointSpecialAbility(e) => translate_flat(e, l),
            Entity::FocusRule(e) => translate_flat(e, l),
            Entity::FoolsHatEnchantment(e) => translate_flat(e, l),
            Entity::GeneralSpecialAbility(e) => translate_flat(e, l),
            Entity::GeodeRitual(e) => translate_flat(e, l),
            Entity::Guideline(e) => translate_flat(e, l),
            Entity::HairColor(e) => translate_flat(e, l),
            Entity::Influence(e) => translate_flat(e, l),
            Entity::InstrumentEnchantment(e) => translate_flat(e, l),
            Entity::ItemGroup(e) => translate_flat(e, l),
            Entity::JesterTrick(e) => translate_flat(e, l),
            Entity::KarmaSpecialAbility(e) => translate_flat(e, l),
            Entity::Krallenkettenzauber(e) => translate_flat(e, l),
            Entity::Language(e) => translate_flat(e, l),
            Entity::LiturgicalChant(e) => translate_flat(e, l),
            Entity::LiturgicalChantGroup(e) => translate_flat(e, l),
            Entity::LiturgicalStyleSpecialAbility(e) => translate_flat(e, l),
            Entity::LycantropicGift(e) => translate_flat(e, l),
            Entity::MagicalDance(e) => translate_flat(e, l),
            Entity::MagicalMelody(e) => translate_flat(e, l),
            Entity::MagicalRune(e) => translate_flat(e, l),
            Entity::MagicalSpecialAbility(e) => translate_flat(e, l),
            Entity::MagicalTradition(e) => translate_flat(e, l),
            Entity::MagicalTraditionPlaceholder(e) => translate_flat(e, l),
            Entity::MagicStyleSpecialAbility(e) => translate_flat(e, l),
            Entity::MeleeCombatTechnique(e) => translate_flat(e, l),
            Entity::OptionalRule(e) => translate_flat(e, l),
            Entity::OrbEnchantment(e) => translate_flat(e, l),
            Entity::PactCategory(e) => translate_flat(e, l),
            Entity::PactGift(e) => translate_flat(e, l),
            Entity::Patron(e) => translate_flat(e, l),
            Entity::PatronCategory(e) => translate_flat(e, l),
            Entity::PersonalityTrait(e) => translate_flat(e, l),
            Entity::Poison(e) => translate_flat(e, l),
            Entity::Profession(e) => translate_flat(e, l),
            Entity::Property(e) => translate_flat(e, l),
            Entity::ProtectiveWardingCircleSpecialAbility(e) =>
                translate_flat(e, l),
            Entity::Publication(e) => translate_flat(e, l),
            Entity::Race(e) => translate_flat(e, l),
            Entity::RangedCombatTechnique(e) => translate_flat(e, l),
            Entity::Reach(e) => translate_flat(e, l),
            Entity::Region(e) => translate_flat(e, l),
            Entity::RingEnchantment(e) => translate_flat(e, l),
            Entity::Ritual(e) => translate_flat(e, l),
            Entity::Script(e) => translate_flat(e, l),
            Entity::Sermon(e) => translate_flat(e, l),
            Entity::Service(e) => translate_flat(e, l),
            Entity::SexPractice(e) => translate_flat(e, l),
            Entity::SexSpecialAbility(e) => translate_flat(e, l),
            Entity::SickleRitual(e) => translate_flat(e, l),
            Entity::SikaryanDrainSpecialAbility(e) => translate_flat(e, l),
            Entity::Skill(e) => translate_flat(e, l),
            Entity::SkillGroup(e) => translate_flat(e, l),
            Entity::SkillStyleSpecialAbility(e) => translate_flat(e, l),
            Entity::SocialStatus(e) => translate_flat(e, l),
            Entity::SpecialAbilityGroup(e) => translate_flat(e, l),
            Entity::SpellGroup(e) => translate_flat(e, l),
            Entity::Spell(e) => translate_flat(e, l),
            Entity::SpellSwordEnchantment(e) => translate_flat(e, l),
            Entity::StaffEnchantment(e) => translate_flat(e, l),
            Entity::State(e) => translate_flat(e, l),
            Entity::Subject(e) => translate_flat(e, l),
            Entity::ToyEnchantment(e) => translate_flat(e, l),
            Entity::TradeSecret(e) => translate_flat(e, l),
            Entity::Tribe(e) => translate_flat(e, l),
            Entity::Trinkhornzauber(e) => translate_flat(e, l),
            Entity::VampiricGift(e) => translate_flat(e, l),
            Entity::Vision(e) => translate_flat(e, l),
            Entity::WandEnchantment(e) => translate_flat(e, l),
            Entity::WeaponEnchantment(e) => translate_flat(e, l),
            Entity::ZibiljaRitual(e) => translate_flat(e, l)
        }
    }
}

macro_rules! entity_from {
    ( $x:ident ) => {
        impl From<$x> for Entity {
            fn from(s: $x) -> Entity {
                Entity::$x(s)
            }
        }
    };
}

entity_from!(AdvancedCombatSpecialAbility);
entity_from!(AdvancedKarmaSpecialAbility);
entity_from!(AdvancedMagicalSpecialAbility);
entity_from!(AdvancedSkillSpecialAbility);
entity_from!(Advantage);
entity_from!(AncestorGlyph);
entity_from!(AnimalDisease);
entity_from!(AnimalShapePath);
entity_from!(AnimalShape);
entity_from!(AnimalShapeSize);
entity_from!(AnimalType);
entity_from!(AnimistPower);
entity_from!(ArcaneBardTradition);
entity_from!(ArcaneDancerTradition);
entity_from!(ArcaneOrbEnchantment);
entity_from!(ArmorType);
entity_from!(Aspect);
entity_from!(AttireEnchantment);
entity_from!(Attribute);
entity_from!(BlessedTradition);
entity_from!(Blessing);
entity_from!(BowlEnchantment);
entity_from!(BrawlingSpecialAbility);
entity_from!(Brew);
entity_from!(Cantrip);
entity_from!(CauldronEnchantment);
entity_from!(CeremonialItemSpecialAbility);
entity_from!(Ceremony);
entity_from!(ChronicleEnchantment);
entity_from!(CombatSpecialAbility);
entity_from!(CombatSpecialAbilityGroup);
entity_from!(CombatStyleSpecialAbility);
entity_from!(CombatTechniqueGroup);
entity_from!(CommandSpecialAbility);
entity_from!(Condition);
entity_from!(Culture);
entity_from!(Curriculum);
entity_from!(Curse);
entity_from!(DaggerRitual);
entity_from!(DerivedCharacteristic);
entity_from!(Disadvantage);
entity_from!(Disease);
entity_from!(DominationRitual);
entity_from!(Element);
entity_from!(ElvenMagicalSong);
entity_from!(EquipmentPackage);
entity_from!(ExperienceLevel);
entity_from!(EyeColor);
entity_from!(FamiliarSpecialAbility);
entity_from!(FamiliarsTrick);
entity_from!(FatePointSexSpecialAbility);
entity_from!(FatePointSpecialAbility);
entity_from!(FocusRule);
entity_from!(FoolsHatEnchantment);
entity_from!(GeneralSpecialAbility);
entity_from!(GeodeRitual);
entity_from!(Guideline);
entity_from!(HairColor);
entity_from!(Influence);
entity_from!(InstrumentEnchantment);
entity_from!(ItemGroup);
entity_from!(JesterTrick);
entity_from!(KarmaSpecialAbility);
entity_from!(Krallenkettenzauber);
entity_from!(Language);
entity_from!(LiturgicalChant);
entity_from!(LiturgicalChantGroup);
entity_from!(LiturgicalStyleSpecialAbility);
entity_from!(LycantropicGift);
entity_from!(MagicalDance);
entity_from!(MagicalMelody);
entity_from!(MagicalRune);
entity_from!(MagicalSpecialAbility);
entity_from!(MagicalTradition);
entity_from!(MagicalTraditionPlaceholder);
entity_from!(MagicStyleSpecialAbility);
entity_from!(MeleeCombatTechnique);
entity_from!(OptionalRule);
entity_from!(OrbEnchantment);
entity_from!(PactCategory);
entity_from!(PactGift);
entity_from!(Patron);
entity_from!(PatronCategory);
entity_from!(PersonalityTrait);
entity_from!(Poison);
entity_from!(Profession);
entity_from!(Property);
entity_from!(ProtectiveWardingCircleSpecialAbility);
entity_from!(Publication);
entity_from!(Race);
entity_from!(RangedCombatTechnique);
entity_from!(Reach);
entity_from!(Region);
entity_from!(RingEnchantment);
entity_from!(Ritual);
entity_from!(Script);
entity_from!(Sermon);
entity_from!(Service);
entity_from!(SexPractice);
entity_from!(SexSpecialAbility);
entity_from!(SickleRitual);
entity_from!(SikaryanDrainSpecialAbility);
entity_from!(Skill);
entity_from!(SkillGroup);
entity_from!(SkillStyleSpecialAbility);
entity_from!(SocialStatus);
entity_from!(SpecialAbilityGroup);
entity_from!(SpellGroup);
entity_from!(Spell);
entity_from!(SpellSwordEnchantment);
entity_from!(StaffEnchantment);
entity_from!(State);
entity_from!(Subject);
entity_from!(ToyEnchantment);
entity_from!(TradeSecret);
entity_from!(Tribe);
entity_from!(Trinkhornzauber);
entity_from!(VampiricGift);
entity_from!(Vision);
entity_from!(WandEnchantment);
entity_from!(WeaponEnchantment);
entity_from!(ZibiljaRitual);

impl<T> From<&T> for Entity
where
    T: Clone,
    Entity: From<T>
{
    fn from(t: &T) -> Entity {
        Entity::from(t.clone())
    }
}
