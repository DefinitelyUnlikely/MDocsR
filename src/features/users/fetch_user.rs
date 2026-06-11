use sqlx::{query_as, PgPool};
use crate::common::error::Error;
use crate::features::users::user::User;

pub async fn fetch_user_by_id(id: String, pool: &PgPool) -> Result<User, Error> {
    let result = query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await;
}