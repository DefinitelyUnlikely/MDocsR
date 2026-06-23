use crate::common::error::Error;
use crate::features::users::user::User;
use sqlx::PgPool;

pub async fn fetch_user_by_id(id: &str, pool: &PgPool) -> Result<User, Error> {
    println!("Fetching user by id {}", id);
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}
