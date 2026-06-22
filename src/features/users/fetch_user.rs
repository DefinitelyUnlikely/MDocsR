use crate::common::error::Error;
use crate::features::users::user::User;
use sqlx::{PgPool, query_as};

pub async fn fetch_user_by_id(id: String, pool: &PgPool) -> Result<User, Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}
