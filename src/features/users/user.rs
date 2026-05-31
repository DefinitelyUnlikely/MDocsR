use chrono::{DateTime, Utc};

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