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
            expires: Utc::now() + Duration::days(7),
        }
    }

    pub fn generate_token_value() -> String {
        let mut bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        hex::encode(bytes)
    }
}

