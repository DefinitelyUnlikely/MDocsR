use crate::common::error::Error;
use crate::features::users;
use chrono::{DateTime, Duration, Utc};
use rand::prelude::*;
use sqlx::PgPool;
use std::result;

#[derive(sqlx::FromRow)]
pub struct RefreshToken {
    pub token: String,
    pub user_id: String,
    pub expires: DateTime<Utc>,
}

impl RefreshToken {
    pub fn new(user_id: String) -> Self {
        RefreshToken {
            token: Self::generate_token_value(),
            user_id,
            expires: Utc::now() + Duration::days(7), // TODO: exchange for config
        }
    }

    pub fn generate_token_value() -> String {
        let mut bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        hex::encode(bytes)
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires
    }
}

pub async fn find_refresh_token(value: String, pool: &PgPool) -> Option<RefreshToken> {
    let token = sqlx::query_as!(
        RefreshToken,
        "SELECT * FROM refresh_tokens WHERE token = $1",
        value
    )
    .fetch_one(pool)
    .await;

    if token.is_err() {
        None
    } else {
        Some(token.unwrap())
    }
}

/// Validates and consumes a refresh token. Returns a Result<bool, Error>
/// to indicate if the token was valid or not.
pub async fn consume_refresh_token(token: RefreshToken, pool: &PgPool) -> Result<bool, Error> {
    let delete_result = sqlx::query!("DELETE FROM refresh_tokens WHERE token = $1", token.token)
        .execute(pool)
        .await?;

    if RefreshToken::is_expired(&token) {
        return Ok(false);
    }

    let user = users::fetch_user::fetch_user_by_id(token.user_id, pool).await;

    if user.is_err() { Ok(false) } else { Ok(true) }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token_value_length() {
        let token = RefreshToken::generate_token_value();
        // 32 bytes hex encoded = 64 characters
        assert_eq!(token.len(), 64);
    }

    #[test]
    fn test_new_refresh_token_expiration() {
        let user_id = "user-123".to_string();
        let token = RefreshToken::new(user_id.clone());

        assert_eq!(token.user_id, user_id);
        // Expiration should be in the future
        assert!(token.expires > Utc::now());
    }

    #[test]
    fn test_is_expired() {
        let user_id = "user-123".to_string();
        let token = RefreshToken::new(user_id.clone());
        let expired_token = RefreshToken {
            token: RefreshToken::generate_token_value(),
            user_id,
            expires: Utc::now() + Duration::days(-7), // TODO: exchange for config
        };

        assert!(!token.is_expired());
        assert!(expired_token.is_expired());
    }
}
