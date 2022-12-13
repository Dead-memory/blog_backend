use sea_orm_migration::prelude::*;

use super::m20221213_000004_add_article_table::Article;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            sea_query::Table::alter()
                .table(Article::Table)
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("creation_date")).date().not_null()
                ).add_column_if_not_exists(
                    ColumnDef::new(Alias::new("edition_date")).date().not_null()
                ).to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Article::Table).to_owned()).await
    }
}

