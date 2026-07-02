use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use crate::AppState;
use crate::common::dtos::passkeys::RegisterPasskeyRequest;
use crate::features::users::db::PostgresUserRepository;

pub async fn register_start(Json(payload): Json<RegisterPasskeyRequest>, State(state): State<AppState>) -> impl IntoResponse {
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
