use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum CombatTechniqueId {
    MeleeCombatTechnique(u32),
    RangedCombatTechnique(u32)
}
