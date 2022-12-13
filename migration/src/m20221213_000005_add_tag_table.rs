use sea_orm_migration::prelude::*;

use super::m20221213_000004_add_article_table::Article;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Tag::Table)
                .if_not_exists()
                .col(ColumnDef::new(Tag::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Tag::Name).string().not_null())
                .col(ColumnDef::new(Tag::Description).string().not_null())
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(TagsArticles::Table)
                .if_not_exists()
                .col(ColumnDef::new(TagsArticles::TagId).integer().not_null())
                .col(ColumnDef::new(TagsArticles::ArticleId).integer().not_null())
                .primary_key(Index::create().col(TagsArticles::TagId).col(TagsArticles::ArticleId))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-tags-articles-tag")
                        .from(TagsArticles::Table, TagsArticles::TagId)
                        .to(Tag::Table, Tag::Id)
                ).foreign_key(
                    ForeignKey::create()
                        .name("fk-tags-articles-article")
                        .from(TagsArticles::Table, TagsArticles::ArticleId)
                        .to(Article::Table, Article::Id)
                )
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TagsArticles::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Tag::Table).to_owned()).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Tag {
    Table,
    Id,
    Name,
    Description
}

#[derive(Iden)]
pub enum TagsArticles {
    Table,
    TagId,
    ArticleId
}

