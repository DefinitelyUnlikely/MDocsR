use chrono::{DateTime, Utc};
use textnonce::TextNonce;

pub struct RegistrationNonce {
    pub nonce: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl RegistrationNonce {
    pub fn new(email: String) -> RegistrationNonce {
        Self {
            nonce: TextNonce::new().to_string(),
            email,
            created_at: Utc::now(),
        }

    }
}