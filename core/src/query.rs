use ::entity::*;
use sea_orm::*;

pub struct Query;

impl Query {
    // -=Users=-
    pub async fn find_user_by_id(
        db: &DbConn, 
        id: i32
    ) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_id(id).one(db).await
    }

    pub async fn find_user_by_username(
        db: &DbConn, 
        username: &String
    ) -> Result<Vec<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Pseudo.like(username))
            .all(db)
            .await
    }

    // -=Sessions=-
    pub async fn remove_session_by_token(
        db: &DbConn, 
        token: String
    ) -> Result<DeleteResult, DbErr> {
        session::Entity::delete_many()
        .filter(session::Column::Token.contains(token.as_str()))
        .exec(db)
        .await
    }

    pub async fn find_user_by_token(
        db: &DbConn,
        token: &String
    ) -> Result<Option<user::Model>, DbErr> {
        let user_and_token = session::Entity::find()
            .filter(session::Column::Token.like(token))
            .find_also_related(user::Entity)
            .one(db)
            .await?;
            
        match user_and_token {
            Option::Some(x) => Result::Ok(x.1),
            Option::None => Result::Ok(Option::None)
        }
    }
}