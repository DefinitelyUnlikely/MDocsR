use crate::common::auth::tokens::refresh_token::refresh_token::RefreshToken;
use sqlx::{Error, PgPool};

/// A repository implementing CRD. Update function does
/// not exist as to enforce token rotation.
pub struct RefreshTokenRepository {
    pool: PgPool,
}

impl RefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Saves a refresh token struct and returns a boolean value
    /// of true if rows affected equals 1, otherwise false.
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

    /// Attempts to retrieve a refresh token from the repository.
    /// Returns an optional RefreshToken or an error.
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

    /// Deletes a refresh token based on the value of the token.
    /// Returns rows affected.
    pub async fn delete_refresh_token(&self, value: &str) -> Result<u64, Error> {
        let result = sqlx::query!("DELETE FROM refresh_tokens WHERE token = $1", value)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
