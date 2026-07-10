use crate::common::error::Error;
use crate::features::documents::document::Document;
use sqlx::PgPool;

pub trait DocumentRepository: Send + Sync {
    async fn save(&self, document: Document) -> Result<(), Error>;

    async fn get_by_id(&self, document_id: &str) -> Result<Option<Document>, Error>;
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

    async fn get_by_id(&self, document_id: &str) -> Result<Option<Document>, Error> {
        let document = sqlx::query_as!(
            Document,
            "SELECT * FROM documents WHERE id = $1",
            document_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(document)
    }

    async fn get_all_user_documents(&self, user_id: &str) -> Result<Vec<Document>, Error> {
        let docs = sqlx::query_as!(
            Document,
            "SELECT * FROM documents WHERE user_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(docs)
    }

    async fn get_all(&self) -> Result<Vec<Document>, Error> {
        let docs = sqlx::query_as!(Document, "SELECT * FROM documents")
            .fetch_all(&self.pool)
            .await?;

        Ok(docs)
    }

    async fn update(&self, document: Document) -> Result<bool, Error> {
        let res = sqlx::query!(
            "UPDATE documents SET title = $1, is_tombstone = $2, updated_at = $3 WHERE id = $4",
            document.title,
            document.is_tombstone,
            document.updated_at, // Do we want to enforce this in the data layer instead of letting the object dictate this?
            document.id
        )
        .execute(&self.pool)
        .await?;

        if res.rows_affected() == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn delete(&self, document_id: &str) -> Result<(), Error> {}
}
