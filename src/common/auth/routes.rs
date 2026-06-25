use axum::{
    routing::{get, post},
    Router,
};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use crate::{AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/refresh-tokens", post(refresh_tokens))
}

async fn refresh_tokens(jar: CookieJar) -> impl IntoResponse {
    if let Some(token) = jar.get("refresh_token") {
        // TODO: Implement this branch
        let new_jar = jar
            .add(Cookie::new("access_token", "placeholder"))
            .add(Cookie::new("refresh_token", "placeholder"));

        (new_jar, (StatusCode::OK, "Token refreshed")).into_response()
    } else {
        (StatusCode::BAD_REQUEST, "No refresh token in cookies").into_response()
    }
}