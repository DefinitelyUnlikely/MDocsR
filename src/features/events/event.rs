use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Event {
    pub id: String,
    pub user_id: String,
    pub document_id: String,
    pub payload: Vec<u8>,
    pub created_at: DateTime<Utc>,
}
