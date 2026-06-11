use sqlx::{query_as, PgPool};
use crate::common::error::Error;
use crate::features::users::user::User;

pub async fn fetch_user_by_id(id: String, pool: &PgPool, include_info: bool) -> Result<User, Error> {
    if include_info {}
    
    let fetch = fetch_user_without_info(id, pool).await;
    match fetch {
        Ok(user) => Ok(user),
        Err(err) => Err(err.into()),
    }
}

async fn fetch_user_with_info(id: String, pool: &PgPool) -> Result<User, Error> {
    todo!()
}

async fn fetch_user_without_info(id: String, pool: &PgPool) -> Result<User, sqlx::Error> {
    let result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(pool)
        .await;
    result
}