use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Snapshot {
    pub id: i32,
    pub document_id: String,
    pub user_id: String,
    pub data: Vec<u8>,
    pub created_at: DateTime<Utc>,
}
