use crate::AppState;
use crate::common::auth::extractor::AuthenticatedUser;
use crate::common::error::Error;
use crate::features::documents::create::new_document::new_document_handler;
use axum::Router;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;

pub fn documents_router() -> Router<AppState> {
    Router::new().route("/new", post(new_document_handler))
}
