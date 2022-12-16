use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}