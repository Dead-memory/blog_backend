use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,

    pub article_id: i64,

    pub author_id: i64,
    pub author_pseudo: String,

    pub content: String
}