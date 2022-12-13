use sea_orm_migration::prelude::*;

use super::m20221211_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Session::Table)
                .if_not_exists()
                .col(ColumnDef::new(Session::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Session::UserId).integer().not_null())
                .col(ColumnDef::new(Session::Token).string().not_null())
                .col(ColumnDef::new(Session::CreationDate).date().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-session-user")
                        .from(Session::Table, Session::UserId)
                        .to(User::Table, User::Id)
                )
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Session::Table).to_owned()).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Session {
    Table,
    Id,
    UserId,
    Token,
    CreationDate
}

