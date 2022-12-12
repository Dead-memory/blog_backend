use ::entity::{user, user::Entity as Post};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        form_data: user::Model
    ) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            pseudo: Set(form_data.pseudo.to_owned()),
            about: Set(form_data.about.to_owned()),

            inscription_date: Set(form_data.inscription_date.to_owned()),
            last_connection_date: Set(form_data.last_connection_date.to_owned()),

            hashed_password: Set(form_data.hashed_password.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}