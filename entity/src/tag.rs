//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::article::Entity> for Entity {
    fn to() -> RelationDef {
        super::tags_articles::Relation::Article.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::tags_articles::Relation::Tag.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
