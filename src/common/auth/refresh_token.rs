use chrono::{DateTime, Duration, Utc};
use rand::prelude::*;

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
