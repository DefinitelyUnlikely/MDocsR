use axum::extract::State;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::post;
use crate::AppState;
use crate::common::auth::extractor::AuthenticatedUser;
use crate::common::error::Error;

pub fn documents_router() -> Router<AppState> {
    Router::new()
        .route("/new", post(new_document))
}

async fn new_document(user: AuthenticatedUser, State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    
}