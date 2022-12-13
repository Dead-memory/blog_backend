use sea_orm_migration::prelude::*;

use super::m20221211_000001_create_user_table::User;
use super::m20221213_000004_add_article_table::Article;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Comment::Table)
                .if_not_exists()
                .col(ColumnDef::new(Comment::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Comment::ArticleId).integer().not_null())
                .col(ColumnDef::new(Comment::UserId).integer().not_null())
                .col(ColumnDef::new(Comment::PublishDate).date().not_null())
                .col(ColumnDef::new(Comment::ModifyDate).date().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-comment-article")
                        .from(Comment::Table, Comment::ArticleId)
                        .to(Article::Table, Article::Id)
                ).foreign_key(
                    ForeignKey::create()
                        .name("fk-comment-user")
                        .from(Comment::Table, Comment::UserId)
                        .to(User::Table, User::Id)
                )
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Comment::Table).to_owned()).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Comment {
    Table,
    Id,
    ArticleId,
    UserId,
    PublishDate,
    ModifyDate
}

