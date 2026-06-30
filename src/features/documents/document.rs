use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Document {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub is_tombstone: bool,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}