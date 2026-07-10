use crate::common::error::Error;
use sqlx::PgPool;
use crate::features::documents::document::Document;

pub trait DocumentRepository: Send + Sync {
    async fn save(&self, document: Document) -> Result<(), Error>;

    async fn get_by_id(&self, document_id: &str) -> Result<Document, Error>;
    async fn get_all_user_documents(&self, user_id: &str) -> Result<Vec<Document>, Error>;
    async fn get_all(&self) -> Result<Vec<Document>, Error>;

    async fn update(&self, document: Document) -> Result<bool, Error>;
    async fn delete(&self, document_id: &str) -> Result<(), Error>;
}

pub struct PostgresDocumentRepository {
    pool: PgPool,
}

impl PostgresDocumentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl DocumentRepository for PostgresDocumentRepository {
    
    async fn save(&self, document: Document) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO documents (id, user_id, title, is_tombstone, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)",
        document.id,
        document.user_id,
        document.title,
        document.is_tombstone,
        document.created_at,
        document.updated_at)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
}