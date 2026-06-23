use crate::common::auth::refresh_token::refresh_token::RefreshToken;
use sqlx::PgPool;


struct RefreshTokenRepository {
    pool: PgPool,
}

impl RefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_refresh_token(&self, value: &str) -> Option<RefreshToken> {
        println!("Fetching refresh token with value {}", value);
        let token = sqlx::query_as!(
        RefreshToken,
        "SELECT * FROM refresh_tokens WHERE token = $1",
        value
    )
            .fetch_one(&self.pool)
            .await;

        if token.is_err() {
            None
        } else {
            Some(token.unwrap())
        }
    }
}

