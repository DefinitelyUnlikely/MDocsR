use crate::common::error::Error;
use crate::features::users::user::User;
use sqlx::PgPool;

struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn fetch_user_by_id(&self, id: &str) -> Result<Option<User>, Error> {
        println!("Fetching user by id {}", id);
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}
