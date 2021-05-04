use serde::{Deserialize, Serialize};

pub mod culture;
pub mod profession;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PackageSkill {
    pub id: u32,

    /// The skill points for the respective skill you get for buying the
    /// package.
    pub value: u32
}
