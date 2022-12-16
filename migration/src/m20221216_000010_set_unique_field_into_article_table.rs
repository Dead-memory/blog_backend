use sea_orm_migration::prelude::*;

use super::m20221213_000004_add_article_table::Article;

#[derive(DeriveMigrationName)]
pub struct Migration;

const PICTURE_URL_FIELD_NAME: &'static str = "picture_url";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        manager.alter_table(
            sea_query::Table::alter()
                .table(Article::Table)
                .modify_column(ColumnDef::new(Alias::new("title"))
                    .string()
                    .not_null()
                    .unique_key()
                )
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            sea_query::Table::alter()
                .table(Article::Table)
                .modify_column(ColumnDef::new(Alias::new("title"))
                    .string()
                    .not_null()
                )
                .to_owned()
        ).await
    }
}

