use sea_orm_migration::prelude::*;

use super::m20221211_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Role::Table)
                .if_not_exists()
                .col(ColumnDef::new(Role::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Role::Name).string().not_null())
                .col(ColumnDef::new(Role::Description).string().not_null())
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(RolesUsers::Table)
                .if_not_exists()
                .col(ColumnDef::new(RolesUsers::RoleId).integer().not_null())
                .col(ColumnDef::new(RolesUsers::UserId).integer().not_null())
                .primary_key(Index::create().col(RolesUsers::RoleId).col(RolesUsers::UserId))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(RolesUsers::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Role::Table).to_owned()).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Role {
    Table,
    Id,
    Name,
    Description
}

#[derive(Iden)]
pub enum RolesUsers {
    Table,
    RoleId,
    UserId
}

