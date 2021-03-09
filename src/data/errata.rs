use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Erratum {
    pub date: String,
    pub description: String
}

pub type Errata = Vec<Erratum>;
