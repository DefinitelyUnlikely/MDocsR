use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Document {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub is_tombstone: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


impl Document {
    pub(crate) fn new(user_id: String, title: String) -> Self {
        Document {
            id: Uuid::new_v4().to_string(),
            user_id,
            title,
            is_tombstone: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}