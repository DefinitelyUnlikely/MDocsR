use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
    pub id: String,
    pub email: String,
    pub password: String, // should always be stored as the hash
    pub salt: String,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct UserInfo {
    pub id: String,
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub country: String,
    pub city: String,
    pub state: Option<String>,
    pub address: String,
    pub postal_code: String,
}

pub struct UserWithInfo {
    pub user: User,
    pub info: UserInfo,
}

impl User {
    pub fn new(email: String, password: String, salt: String) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            email,
            password,
            salt,
            email_verified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn hash_password(&mut self) {
        self.password = bcrypt::hash(&self.password, bcrypt::DEFAULT_COST).unwrap();
    }
}