use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use crate::AppState;
use crate::common::dtos::passkeys::RegisterPasskeyRequest;
use crate::features::users::db::PostgresUserRepository;
use regex::Regex;
use crate::common::error::Error;

pub async fn register_start(Json(payload): Json<RegisterPasskeyRequest>, State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    // start by doing everything in the handler
    // and we can separate the concerns afterward.
    
    // Let's start with using a simple regex to check if the email
    // is at least email-ish.
    let re = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap();
    
    let is_match = re.is_match(&payload.email);
    if !is_match {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    }
    
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());
    
}

pub async fn register_finish() -> impl IntoResponse {
    "Pong".to_string()
}

pub async fn login_start() -> impl IntoResponse {
    "Pong".to_string()
}
pub async fn login_finish() -> impl IntoResponse {
    "Pong".to_string()
}

pub async fn add_passkey_start() -> impl IntoResponse {
    "Pong".to_string()
}
pub async fn add_passkey_finish() -> impl IntoResponse {
    "Pong".to_string()
}
