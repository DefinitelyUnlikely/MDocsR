use crate::common::error::Error;
use sqlx::PgPool;
use crate::features::documents::document::Document;

pub trait DocumentRepository: Send + Sync {
    fn save(&self, document: Document) -> Result<(), Error>;
    
    fn get_by_id(&self, document_id: &str) -> Result<Document, Error>;
    fn get_all_user_documents(&self, user_id: &str) -> Result<Vec<Document>, Error>;
    fn get_all(&self) -> Result<Vec<Document>, Error>;
    
    fn update(&self, document: Document) -> Result<bool, Error>;
    fn delete(&self, document_id: &str) -> Result<(), Error>;
}