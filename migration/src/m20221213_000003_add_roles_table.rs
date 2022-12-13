use sea_orm_migration::prelude::*;

use super::m20221212_000002_create_role_table::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.exec_stmt(
            Query::insert()
                .into_table(Role::Table)
                .columns([Role::Name, Role::Description])
                .values_panic(["User".into(), "Utilisateur lambda".into()])
                .values_panic(["Blogueur".into(), "Utilisateur postant du contenue".into()])
                .values_panic(["Admin".into(), "Gérants des articles de blog".into()])
                .to_owned()
        ).await?;

        Ok(())
    }
}

