use serde::{Serialize};
//use rocket::serde::*;

#[derive(Debug, Serialize)]
#[serde(crate = "serde")]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inscription_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connection_date: Option<String>
}

impl Default for User {
    fn default() -> Self {
        User {
            id: Option::None,
            pseudo: Option::None,
            about: Option::None,
            inscription_date: Option::None,
            last_connection_date: Option::None
        }
    }
}