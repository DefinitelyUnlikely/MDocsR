use crate::common::auth::refresh_token::refresh_token::RefreshToken;
use sqlx::{Error, PgPool};

pub struct RefreshTokenRepository {
    pool: PgPool,
}

impl RefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_refresh_token(&self, token: &RefreshToken) -> Result<bool, Error> {
        println!("Saving refresh token");

        let result = sqlx::query!(
            "INSERT INTO refresh_tokens (token, user_id, expires) VALUES ($1, $2, $3)",
            token.token,
            token.user_id,
            token.expires
        ).execute(&self.pool).await?;

        Ok(result.rows_affected() == 1)
    }

    pub async fn find_refresh_token(&self, value: &str) -> Result<Option<RefreshToken>, Error> {
        println!("Fetching refresh token with value {}", value);
        let token = sqlx::query_as!(
            RefreshToken,
            "SELECT * FROM refresh_tokens WHERE token = $1",
            value
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    pub async fn delete_refresh_token(&self, token: &RefreshToken) -> Result<u64, Error> {
        let result = sqlx::query!("DELETE FROM refresh_tokens WHERE token = $1", token.token)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
