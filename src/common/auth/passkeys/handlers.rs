use axum::response::IntoResponse;

pub async fn register_start() -> impl IntoResponse {
    "Pong".to_string()
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
