use crate::AppState;
use crate::common::auth::extractor::AuthenticatedUser;
use crate::common::error::Error;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::features::documents::document::Document;
use crate::features::documents::repository::DocumentRepository;

#[derive(Deserialize)]
pub struct NewDocumentRequest {
    pub name: String,
}

/// Route handler for creating new documents. Takes a name in a json request body.
pub async fn new_document_handler(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(request): Json<NewDocumentRequest>,
) -> Result<impl IntoResponse, Error> {

}


pub async fn create_new_document(repo: &impl DocumentRepository, user_id: String, name: String) -> Result<impl IntoResponse, Error> {
    Document::
}