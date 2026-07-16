use axum::extract::State;
use axum::response::IntoResponse;
use axum::Router;
use crate::AppState;
use crate::common::auth::extractor::AuthenticatedUser;
use crate::common::error::Error;

pub fn documents_router() -> Router<AppState> {
    Router::new()
        .route("/new")
}

async fn new_document(user: AuthenticatedUser, State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    
}