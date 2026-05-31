use chrono::{DateTime, Utc};

pub struct Snapshot {
    pub id: String,
    pub document_id: String,
    pub user_id: String,
    pub data: Vec<u8>,
    pub created_at: DateTime<Utc>,
}