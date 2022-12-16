use serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "serde")]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub description: String
}