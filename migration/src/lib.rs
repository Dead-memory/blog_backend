pub use sea_orm_migration::prelude::*;

mod m20221211_000001_create_user_table;
mod m20221212_000002_create_role_table;
mod m20221212_000003_create_users_roles_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221211_000001_create_user_table::Migration),
            Box::new(m20221212_000002_create_role_table::Migration)
        ]
    }
}
