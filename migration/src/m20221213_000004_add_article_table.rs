use sea_orm_migration::prelude::*;

use super::m20221211_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Article::Table)
                .if_not_exists()
                .col(ColumnDef::new(Article::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Article::UserId).integer().not_null())
                .col(ColumnDef::new(Article::Title).string().not_null())
                .col(ColumnDef::new(Article::Content).string().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-article-user")
                        .from(Article::Table, Article::UserId)
                        .to(User::Table, User::Id)
                )
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Article::Table).to_owned()).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Article {
    Table,
    Id,
    UserId,
    Title,
    Content
}

