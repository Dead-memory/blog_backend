use serde::{Serialize, Deserialize};
use super::tags;

#[derive(Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Article {
    pub id: i64,

    pub author_id: i64,
    pub author_pseudo: String,

    pub title: String,
    pub content: String,

    pub creation_date: String,
    pub tags: Vec<tags::Tag>,
    pub picture_url: String
}