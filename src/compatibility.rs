use crate::error::OptolithDataResult;
use crate::id::{Category, Id};
use crate::util;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

const ADVANCED_COMBAT_SPECIAL_ABILITIES_FILE: &str =
    "AdvancedCombatSpecialAbilitiesIdMap.yml";
const ADVANCED_KARMA_SPECIAL_ABILITIES_FILE: &str =
    "AdvancedKarmaSpecialAbilitiesIdMap.yml";
const ADVANCED_MAGICAL_SPECIAL_ABILITIES_FILE: &str =
    "AdvancedMagicalSpecialAbilitiesidMap.yml";
const ADVANCED_SKILL_SPECIAL_ABILITIES_FILE: &str =
    "AdvancedSkillSpecialAbilitiesIdMap.yml";
const ADVANTAGES_FILE: &str = "AdvantagesIdMap.yml";
const ANCESTOR_GLYPHS_FILE: &str = "AncestorGlyphsIdMap.yml";
const ANIMIST_POWERS_FILE: &str = "AnimistPowersIdMap.yml";
const ARCANE_ORB_ENCHANTMENTS_FILE: &str =
    "ArcaneOrbEnchantmentsIdMap.yml";
const ASPECTS_FILE: &str = "AspectsIdMap.yml";
const ATTIRE_ENCHANTMENTS_FILE: &str = "AttireEnchantmentsIdMap.yml";
const BLESSED_TRADITIONS_FILE: &str = "BlessedTraditionsIdMap.yml";
const BOWL_ENCHANTMENTS_FILE: &str = "BowlEnchantmentsIdMap.yml";
const BRAWLING_SPECIAL_ABILTIES_FILE: &str =
    "BrawlingSpecialAbilitiesIdMap.yml";
const CAULDRON_ENCHANTMENTS_FILE: &str =
    "CauldronEnchantmentsIdMap.yml";
const CEREMONIAL_ITEM_SPECIAL_ABILITIES_FILE: &str =
    "CeremonialItemSpecialAbilitiesIdMap.yml";
const CEREMONIES_FILE: &str = "CeremoniesIdMap.yml";
const CHRONICLE_ENCHANTMENTS_FILE: &str = "ChronicleEnchantmentsIdMap.yml";
const COMBAT_SPECIAL_ABILITIES_FILE: &str =
    "CombatSpecialAbilitiesIdMap.yml";
const COMBAT_STYLE_SPECIAL_ABILITIES_FILE: &str =
    "CombatStyleSpecialAbilitiesIdMap.yml";
const COMMAND_SPECIAL_ABILITIES_FILE: &str =
    "CommandSpecialAbilitiesIdMap.yml";
const CONDITIONS_FILE: &str = "ConditionsIdMap.yml";
const CULTURES_FILE: &str = "CulturesIdMap.yml";
const CURSES_FILE: &str = "CursesIdMap.yml";
const DAGGER_RITUALS_FILE: &str = "DaggerRitualsIdMap.yml";
const DISADVANTAGES_FILE: &str = "DisadvantagesIdMap.yml";
const DISEASES_FILE: &str = "DiseasesIdMap.yml";
const DOMINATION_RITUALS_FILE: &str = "DominationRitualsIdMap.yml";
const ELVEN_MAGICAL_SONGS_FILE: &str = "ElvenMagicalSongsIdMap.yml";
const FAMILIAR_SPECIAL_ABILITIES_FILE: &str =
    "FamiliarSpecialAbilitiesIdMap.yml";
const FATE_POINT_SEX_SPECIAL_ABILITIES_FILE: &str =
    "FatePointSexSpecialAbilitiesIdMap.yml";
const FATE_POINT_SPECIAL_ABILITIES_FILE: &str =
    "FatePointSpecialAbilitiesIdMap.yml";
const FOOLS_HAT_ENCHANTMENTS_FILE: &str =
    "FoolsHatEnchantmentsIdMap.yml";
const GENERAL_SPECIAL_ABILITIES_FILE: &str =
    "GeneralSpecialAbilitiesIdMap.yml";
const GEODE_RITUALS_FILE: &str = "GeodeRitualsIdMap.yml";
const INSTRUMENT_ENCHANTMENTS_FILE: &str =
    "InstrumentEnchantmentsIdMap.yml";
const JESTER_TRICKS_FILE: &str = "JesterTricksIdMap.yml";
const KARMA_SPECIAL_ABILITIES_FILE: &str =
    "KarmaSpecialAbilitiesIdMap.yml";
const LITURGICAL_CHANTS_FILE: &str = "LiturgicalChantsIdMap.yml";
const LITURGICAL_STYLE_SPECIAL_ABILITIES_FILE: &str =
    "LiturgicalStyleSpecialAbilitiesIdMap.yml";
const LYCANTROPIC_GIFTS_FILE: &str =
    "LycantropicGiftsIdMap.yml";
const MAGICAL_DANCES_FILE: &str = "MagicalDancesIdMap.yml";
const MAGICAL_MELODIES_FILE: &str = "MagicalMelodiesIdMap.yml";
const MAGICAL_SPECIAL_ABILITIES_FILE: &str =
    "MagicalSpecialAbilitiesIdMap.yml";
const MAGICAL_TRADITIONS_FILE: &str = "MagicalTraditionsIdMap.yml";
const MAGIC_STYLE_SPECIAL_ABILITIES_FILE: &str =
    "MagicStyleSpecialAbilitiesIdMap.yml";
const MELEE_COMBAT_TECHNIQUES_FILE: &str =
    "MeleeCombatTechniquesIdMap.yml";
const ORB_ENCHANTMENTS_FILE: &str = "OrbEnchantmentsIdMap.yml";
const PACT_GIFTS_FILE: &str = "PactGiftsIdMap.yml";
const POISONS_FILE: &str = "PoisonsIdMap.yml";
const PROTECTIVE_WARDING_CIRCLE_SPECIAL_ABILITIES_FILE: &str =
    "ProtectiveWardingCircleSpecialAbilitiesIdMap.yml";
const RANGED_COMBAT_TECHNIQUES_FILE: &str =
    "RangedCombatTechniquesIdMap.yml";
const RING_ENCHANTMENTS_FILE: &str = "RingEnchantmentsIdMap.yml";
const RITUALS_FILE: &str = "RitualsIdMap.yml";
const SERMONS_FILE: &str = "SermonsIdMap.yml";
const SEX_SPECIAL_ABILITIES_FILE: &str =
    "SexSpecialAbilitiesIdMap.yml";
const SICKLE_RITUALS_FILE: &str = "SickleRitualsIdMap.yml";
const SIKARYAN_DRAIN_SPECIAL_ABILITIES_FILE: &str =
    "SikaryanDrainSpecialAbilitiesIdMap.yml";
const SKILL_STYLE_SPECIAL_ABILITIES_FILE: &str =
    "SkillStyleSpecialAbilititesIdMap.yml";
const SPELLS_FILE: &str = "SpellsIdMap.yml";
const SPELL_SWORD_ENCHANTMENTS_FILE: &str =
    "SpellSwordEnchantmentsIdMap.yml";
const STAFF_ENCHANTMENTS_FILE: &str = "StaffEnchantmentsIdMap.yml";
const STATES_FILE: &str = "StatesIdMap.yml";
const TOY_ENCHANTMENTS_FILE: &str = "ToyEnchantmentsIdMap.yml";
const VAMPIRIC_GIFTS_FILE: &str = "VampiricGiftsIdMap.yml";
const VISIONS_FILE: &str = "VisionsIdMap.yml";
const WEAPON_ENCHANTMENTS_FILE: &str = "WeaponEnchantmentsIdMap.yml";
const WAND_ENCHANTMENTS_FILE: &str = "WandEnchantmentsIdMap.yml";
const ZIBILJA_RITUALS_FILE: &str = "ZibiljaRitualsIdMap.yml";

/// This struct manages the mappings from old IDs used in character files to
/// new IDs used in the Optolith data files.
#[derive(Deserialize, Serialize)]
pub struct Compatibility {
    str_id_map: HashMap<String, Id>,
    diseases_id_map: HashMap<u32, Id>,
    poisons_id_map: HashMap<u32, Id>
}

struct CompatibilityLoader {
    compat: Compatibility,
    path: String
}

fn load_u32_id_map(map: &mut HashMap<u32, Id>, path: &str, file: &str,
        category: Category) -> OptolithDataResult<()> {
    let file = util::join(path, file);
    let vec: Vec<(u32, u32)> = util::deserialize_yaml_file(&file)?;

    for (old_id, new_id) in vec {
        let id = Id::new(category, new_id);
        map.insert(old_id, id);
    }

    Ok(())
}

impl CompatibilityLoader {
    fn new(path: &str) -> CompatibilityLoader {
        CompatibilityLoader {
            compat: Compatibility {
                str_id_map: HashMap::new(),
                diseases_id_map: HashMap::new(),
                poisons_id_map: HashMap::new()
            },
            path: String::from(path)
        }
    }

    fn load_str_id_map(&mut self, file: &str, category: Category)
            -> OptolithDataResult<()> {
        let file = util::join(&self.path, file);
        let vec: Vec<(String, u32)> = util::deserialize_yaml_file(&file)?;
    
        for (s, i) in vec {
            let id = Id::new(category, i);
            self.compat.str_id_map.insert(s, id);
        }
    
        Ok(())
    }

    fn load_traditions_id_map(&mut self, file: &str, category: Category)
            -> OptolithDataResult<()> {
        let file = util::join(&self.path, file);
        let vec: Vec<(String, Option<u32>, u32)> = util::deserialize_yaml_file(&file)?;

        for (s, _, i) in vec {
            let id = Id::new(category, i);
            self.compat.str_id_map.insert(s, id);
        }

        Ok(())
    }

    fn load_prefix_map(&mut self, max: u32, prefix: &str, category: Category) {
        for i in 1..=max {
            let id = Id::new(category, i);
            self.compat.str_id_map.insert(format!("{}_{}", prefix, max), id);
        }
    }

    fn load_diseases(&mut self) -> OptolithDataResult<()> {
        load_u32_id_map(&mut self.compat.diseases_id_map, &self.path,
            DISEASES_FILE, Category::Diseases)
    }

    fn load_poisons(&mut self) -> OptolithDataResult<()> {
        load_u32_id_map(&mut self.compat.poisons_id_map, &self.path,
            POISONS_FILE, Category::Poisons)
    }
}

impl Compatibility {

    pub fn from_directory(path: &str) -> OptolithDataResult<Compatibility> {
        let mut res = CompatibilityLoader::new(path);

        // TODO Publication (if that is even necessary)

        res.load_str_id_map(ADVANCED_COMBAT_SPECIAL_ABILITIES_FILE,
            Category::AdvancedCombatSpecialAbilities)?;
        res.load_str_id_map(ADVANCED_KARMA_SPECIAL_ABILITIES_FILE,
            Category::AdvancedKarmaSpecialAbilities)?;
        res.load_str_id_map(ADVANCED_MAGICAL_SPECIAL_ABILITIES_FILE,
            Category::AdvancedMagicalSpecialAbilities)?;
        res.load_str_id_map(ADVANCED_SKILL_SPECIAL_ABILITIES_FILE, 
            Category::AdvancedSkillSpecialAbilities)?;
        res.load_str_id_map(ADVANTAGES_FILE, Category::Advantages)?;
        res.load_str_id_map(ANCESTOR_GLYPHS_FILE, Category::AncestorGlyphs)?;
        res.load_str_id_map(ANIMIST_POWERS_FILE, Category::AnimistPowers)?;
        res.load_str_id_map(ARCANE_ORB_ENCHANTMENTS_FILE,
            Category::ArcaneOrbEnchantments)?;
        res.load_str_id_map(ASPECTS_FILE, Category::Aspects)?;
        res.load_str_id_map(ATTIRE_ENCHANTMENTS_FILE,
            Category::AttireEnchantments)?;
        res.load_str_id_map(BOWL_ENCHANTMENTS_FILE,
            Category::BowlEnchantments)?;
        res.load_str_id_map(BRAWLING_SPECIAL_ABILTIES_FILE,
            Category::BrawlingSpecialAbilities)?;
        res.load_str_id_map(CAULDRON_ENCHANTMENTS_FILE,
            Category::CauldronEnchantments)?;
        res.load_str_id_map(CEREMONIAL_ITEM_SPECIAL_ABILITIES_FILE,
            Category::CeremonialItemSpecialAbilities)?;
        res.load_str_id_map(CEREMONIES_FILE, Category::Ceremonies)?;
        res.load_str_id_map(CHRONICLE_ENCHANTMENTS_FILE,
            Category::ChronicleEnchantments)?;
        res.load_str_id_map(COMBAT_SPECIAL_ABILITIES_FILE,
            Category::CombatSpecialAbilities)?;
        res.load_str_id_map(COMBAT_STYLE_SPECIAL_ABILITIES_FILE,
            Category::CombatStyleSpecialAbilities)?;
        res.load_str_id_map(COMMAND_SPECIAL_ABILITIES_FILE,
            Category::CommandSpecialAbilities)?;
        res.load_str_id_map(CONDITIONS_FILE, Category::Conditions)?;
        res.load_str_id_map(CULTURES_FILE, Category::Cultures)?;
        res.load_str_id_map(CURSES_FILE, Category::Curses)?;
        res.load_str_id_map(DAGGER_RITUALS_FILE, Category::DaggerRituals)?;
        res.load_str_id_map(DISADVANTAGES_FILE, Category::Disadvantages)?;
        res.load_str_id_map(DOMINATION_RITUALS_FILE,
            Category::DominationRituals)?;
        res.load_str_id_map(ELVEN_MAGICAL_SONGS_FILE,
            Category::ElvenMagicalSongs)?;
        res.load_str_id_map(FAMILIAR_SPECIAL_ABILITIES_FILE, 
            Category::FamiliarSpecialAbilities)?;
        res.load_str_id_map(FATE_POINT_SEX_SPECIAL_ABILITIES_FILE,
            Category::FatePointSexSpecialAbilities)?;
        res.load_str_id_map(FATE_POINT_SPECIAL_ABILITIES_FILE,
            Category::FatePointSpecialAbilities)?;
        res.load_str_id_map(FOOLS_HAT_ENCHANTMENTS_FILE,
            Category::FoolsHatEnchantments)?;
        res.load_str_id_map(GENERAL_SPECIAL_ABILITIES_FILE,
            Category::GeneralSpecialAbilities)?;
        res.load_str_id_map(GEODE_RITUALS_FILE, Category::GeodeRituals)?;
        res.load_str_id_map(INSTRUMENT_ENCHANTMENTS_FILE,
            Category::InstrumentEnchantments)?;
        res.load_str_id_map(JESTER_TRICKS_FILE, Category::JesterTricks)?;
        res.load_str_id_map(KARMA_SPECIAL_ABILITIES_FILE,
            Category::KarmaSpecialAbilities)?;
        res.load_str_id_map(LITURGICAL_CHANTS_FILE,
            Category::LiturgicalChants)?;
        res.load_str_id_map(LITURGICAL_STYLE_SPECIAL_ABILITIES_FILE,
            Category::LiturgicalStyleSpecialAbilities)?;
        res.load_str_id_map(LYCANTROPIC_GIFTS_FILE,
            Category::LycantropicGifts)?;
        res.load_str_id_map(MAGICAL_DANCES_FILE, Category::MagicalDances)?;
        res.load_str_id_map(MAGICAL_MELODIES_FILE, Category::MagicalMelodies)?;
        res.load_str_id_map(MAGICAL_SPECIAL_ABILITIES_FILE,
            Category::MagicalSpecialAbilities)?;
        res.load_str_id_map(MAGIC_STYLE_SPECIAL_ABILITIES_FILE,
            Category::MagicStyleSpecialAbilities)?;
        res.load_str_id_map(MELEE_COMBAT_TECHNIQUES_FILE,
            Category::MeleeCombatTechniques)?;
        res.load_str_id_map(ORB_ENCHANTMENTS_FILE, Category::OrbEnchantments)?;
        res.load_str_id_map(PACT_GIFTS_FILE, Category::PactGifts)?;
        res.load_str_id_map(PROTECTIVE_WARDING_CIRCLE_SPECIAL_ABILITIES_FILE,
            Category::ProtectiveWardingCircleSpecialAbilities)?;
        res.load_str_id_map(RANGED_COMBAT_TECHNIQUES_FILE,
            Category::RangedCombatTechniques)?;
        res.load_str_id_map(RING_ENCHANTMENTS_FILE,
            Category::RingEnchantments)?;
        res.load_str_id_map(RITUALS_FILE, Category::Rituals)?;
        res.load_str_id_map(SERMONS_FILE, Category::Sermons)?;
        res.load_str_id_map(SEX_SPECIAL_ABILITIES_FILE,
            Category::SexSpecialAbilities)?;
        res.load_str_id_map(SICKLE_RITUALS_FILE, Category::SickleRituals)?;
        res.load_str_id_map(SIKARYAN_DRAIN_SPECIAL_ABILITIES_FILE,
            Category::SikaryanDrainSpecialAbilities)?;
        res.load_str_id_map(SKILL_STYLE_SPECIAL_ABILITIES_FILE,
            Category::SkillStyleSpecialAbilities)?;
        res.load_str_id_map(SPELLS_FILE, Category::Spells)?;
        res.load_str_id_map(SPELL_SWORD_ENCHANTMENTS_FILE,
            Category::SpellSwordEnchantments)?;
        res.load_str_id_map(STAFF_ENCHANTMENTS_FILE,
            Category::StaffEnchantments)?;
        res.load_str_id_map(STATES_FILE, Category::States)?;
        res.load_str_id_map(TOY_ENCHANTMENTS_FILE, Category::ToyEnchantments)?;
        res.load_str_id_map(VAMPIRIC_GIFTS_FILE, Category::VampiricGifts)?;
        res.load_str_id_map(VISIONS_FILE, Category::Visions)?;
        res.load_str_id_map(WEAPON_ENCHANTMENTS_FILE,
            Category::WeaponEnchantments)?;
        res.load_str_id_map(WAND_ENCHANTMENTS_FILE,
            Category::WandEnchantments)?;
        res.load_str_id_map(ZIBILJA_RITUALS_FILE, Category::ZibiljaRituals)?;
        
        res.load_prefix_map(59, "TAL", Category::Skills);

        res.load_traditions_id_map(BLESSED_TRADITIONS_FILE,
            Category::BlessedTraditions)?;
        res.load_traditions_id_map(MAGICAL_TRADITIONS_FILE,
            Category::MagicalTraditions)?;

        res.load_diseases()?;
        res.load_poisons()?;

        Ok(res.compat)
    }

    pub fn from_file(path: &str) -> OptolithDataResult<Compatibility> {
        util::from_compressed_file(path)
    }

    pub fn save_to_file(&self, path: &str) -> OptolithDataResult<()> {
        util::to_compressed_file(self, path)
    }

    pub fn get_str_id(&self, str_id: &str) -> Option<Id> {
        self.str_id_map.get(str_id).cloned()
    }

    pub fn get_disease_id(&self, old_id: u32) -> Option<Id> {
        self.diseases_id_map.get(&old_id).cloned()
    }

    pub fn get_poison_id(&self, old_id: u32) -> Option<Id> {
        self.poisons_id_map.get(&old_id).cloned()
    }
}
