use crate::error::OptolithDataResult;
use crate::Sex;

use serde::Deserialize;

use serde_repr::Deserialize_repr;

use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct AdventurePoints {
    pub total: u32
}

#[derive(Deserialize)]
pub struct AttributeValue {
    pub id: String,
    pub value: u32
}

#[derive(Deserialize)]
pub struct IrredeemablePermanentPoints {
    pub lost: u32
}

#[derive(Deserialize)]
pub struct RedeemablePermanentPoints {
    pub lost: u32,
    pub redeemed: u32
}

#[derive(Deserialize)]
pub struct Attributes {
    pub values: Vec<AttributeValue>,
    pub lp: u32,
    pub ae: u32,
    pub kp: u32,
    #[serde(rename = "permanentLP")]
    pub permanent_lp: IrredeemablePermanentPoints,
    #[serde(rename = "permanentAE")]
    pub permanent_ae: RedeemablePermanentPoints,
    #[serde(rename = "permanentKP")]
    pub permanent_kp: RedeemablePermanentPoints,
    #[serde(rename = "attributeAdjustmentSelected")]
    pub attribute_adjustment_selected: Option<String>
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub enum SelectionId {
    Integer(u32),
    String(String)
}

impl SelectionId {
    pub fn to_u32(&self) -> Option<u32> {
        if let SelectionId::Integer(i) = self {
            Some(*i)
        }
        else {
            None
        }
    }

    pub fn to_str(&self) -> Option<&str> {
        if let SelectionId::String(s) = self {
            Some(s)
        }
        else {
            None
        }
    }
}

#[derive(Deserialize)]
pub struct Activatable {
    pub sid: Option<SelectionId>,
    pub sid2: Option<SelectionId>,
    pub sid3: Option<SelectionId>,
    pub tier: Option<u32>,
    pub cost: Option<u32>
}

pub type Activatables = HashMap<String, Vec<Activatable>>;
pub type Skills = HashMap<String, u32>;

#[derive(Deserialize_repr)]
#[repr(u32)]
pub enum HigherParadeValues {
    Inactive = 0,
    Two = 2,
    Four = 4
}

#[derive(Deserialize_repr)]
#[repr(u32)]
pub enum Phase {
    RCPSelection = 1,
    Creation = 2,
    AfterCreation = 3
}

#[derive(Deserialize)]
pub struct Rules {
    #[serde(rename = "higherParadeValues")]
    pub higher_parade_values: HigherParadeValues,
    #[serde(rename = "attributeValueLimit")]
    pub attribute_value_limit: bool,
    #[serde(rename = "enableAllRuleBooks")]
    pub enable_all_rule_books: Option<bool>,
    #[serde(rename = "enabledRuleBooks")]
    pub enabled_rule_books: Option<Vec<String>>,
    #[serde(rename = "enableLanguageSpecializations")]
    pub enable_language_specializations: bool
}

#[derive(Deserialize)]
pub struct PersonalData {
    pub family: Option<String>,
    #[serde(rename = "placeofbirth")]
    pub place_of_birth: Option<String>,
    #[serde(rename = "dateofbirth")]
    pub date_of_birth: Option<String>,
    pub age: Option<String>,
    #[serde(rename = "haircolor")]
    pub hair_color: Option<u32>,
    #[serde(rename = "eyecolor")]
    pub eye_color: Option<u32>,
    pub size: Option<String>,
    pub weight: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "socialstatus")]
    pub social_status: Option<u32>,
    pub characteristics: Option<String>,
    #[serde(rename = "otherinfo")]
    pub other_info: Option<String>,
    #[serde(rename = "cultureAreaKnowledge")]
    pub culture_area_knowledge: Option<String>
}

#[derive(Deserialize)]
pub struct Item {
    pub id: String,
    pub price: Option<f64>,
    pub weight: Option<f64>
}

#[derive(Deserialize)]
pub struct Belongings {
    pub items: HashMap<String, Item>
}

/// A model of a character as it is stored by Optolith.
#[derive(Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    #[serde(rename = "clientVersion")]
    pub client_version: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    pub locale: Option<String>,
    pub avatar: Option<String>,
    pub ap: AdventurePoints,
    pub r: Option<String>,
    pub rv: Option<String>,
    pub c: Option<String>,
    #[serde(rename = "isCulturalPackageActive")]
    pub is_cultural_package_active: Option<bool>,
    pub p: Option<String>,
    #[serde(rename = "professionName")]
    pub profession_name: Option<String>,
    pub pv: Option<String>,
    pub sex: Sex,
    pub rules: Rules,
    pub phase: Phase,
    pub el: String,
    pub pers: PersonalData,
    pub activatable: Activatables,
    pub attr: Attributes,
    pub talents: Skills,
    pub ct: Skills,
    pub spells: Skills,
    pub cantrips: Vec<String>,
    pub liturgies: Skills,
    pub blessings: Vec<String>,
    pub belongings: Option<Belongings>
}

impl Character {
    pub fn from_file(path: &str) -> OptolithDataResult<Character> {
        let json = fs::read_to_string(path)?;
        Ok(serde_json::from_str::<Character>(&json)?)
    }
}
