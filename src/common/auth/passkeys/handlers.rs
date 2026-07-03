use crate::AppState;
use crate::common::dtos::passkeys::RegisterPasskeyRequest;
use crate::common::error::Error;
use crate::features::users::db::{PostgresUserRepository, UserRepository};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use regex::Regex;
use uuid::Uuid;
use webauthn_rs::fake::WebauthnFakeCredentialGenerator;

pub async fn register_start(
    Json(payload): Json<RegisterPasskeyRequest>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
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

    // If we find a user, they cannot register using this endpoint.
    // but to prevent user enumeration, we store that this session is
    // a dummy registration. We still return a real challenge.
    let (user_id, is_dummy) = match user_repo.fetch_user_by_email(&payload.email).await? {
        Some(user) => (user.id, true),
        None => (Uuid::new_v4().to_string(), false),
    };
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
