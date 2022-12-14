pub use sea_orm_migration::prelude::*;

mod m20221211_000001_create_user_table;
mod m20221212_000002_create_role_table;
mod m20221213_000003_add_roles_table;
mod m20221213_000004_add_article_table;
mod m20221213_000005_add_tag_table;
mod m20221213_000006_add_comment_table;
mod m20221213_000007_add_session_table;
mod m20221213_000008_add_creation_and_edition_row_into_article_table;
mod m20221216_000009_add_picture_url_column_into_article_table;
mod m20221216_000010_set_unique_field_into_article_table;
mod m20221216_000011_set_unique_field_into_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221211_000001_create_user_table::Migration),
            Box::new(m20221212_000002_create_role_table::Migration),
            Box::new(m20221213_000003_add_roles_table::Migration),
            Box::new(m20221213_000004_add_article_table::Migration),
            Box::new(m20221213_000005_add_tag_table::Migration),
            Box::new(m20221213_000006_add_comment_table::Migration),
            Box::new(m20221213_000007_add_session_table::Migration),
            Box::new(m20221213_000008_add_creation_and_edition_row_into_article_table::Migration),
            Box::new(m20221216_000009_add_picture_url_column_into_article_table::Migration),
            Box::new(m20221216_000010_set_unique_field_into_article_table::Migration),
            Box::new(m20221216_000011_set_unique_field_into_user_table::Migration)
        ]
    }
}
