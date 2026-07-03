use crate::AppState;
use crate::common::dtos::passkeys::RegisterPasskeyRequest;
use crate::common::error::Error;
use crate::features::users::db::{PostgresUserRepository, UserRepository};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_session::{Session, SessionNullPool};
use regex::Regex;
use uuid::Uuid;
use webauthn_rs::fake::WebauthnFakeCredentialGenerator;

pub async fn register_start(
    Json(payload): Json<RegisterPasskeyRequest>,
    State(state): State<AppState>,
    session: Session<SessionNullPool>,
    Path(nonce): Path<String>,
) -> Result<impl IntoResponse, Error> {
    // start by doing everything in the handler
    // and we can separate the concerns afterward.
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
