use crate::common::error::Error;
use crate::features::users::user::User;
use sqlx::PgPool;

#[allow(async_fn_in_trait)]
pub trait UserRepository: Send + Sync {
    async fn save_user(&self, user: User) -> Result<(), Error>;
    async fn fetch_user_by_id(&self, id: &str) -> Result<Option<User>, Error>;
    async fn fetch_user_by_email(&self, email: &str) -> Result<Option<User>, Error>;
}

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PostgresUserRepository {
    async fn save_user(&self, user: User) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO users (id, email, created_at, updated_at) VALUES ($1, $2, $3, $4)",
            user.id,
            user.email,
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn fetch_user_by_id(&self, id: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    async fn fetch_user_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}
