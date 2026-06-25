use crate::AppState;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;

pub fn router() -> Router<AppState> {
    Router::new().route("/refresh-tokens", post(refresh_tokens))
}

async fn refresh_tokens(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let Some(token) = jar.get("refresh_token") else {
        return (StatusCode::BAD_REQUEST, "No refresh token in cookies").into_response();
    };

    // TODO: Implement this branch
    let new_jar = jar
        .add(Cookie::new("access_token", "placeholder"))
        .add(Cookie::new("refresh_token", "placeholder"));

    (new_jar, (StatusCode::OK, "Token refreshed")).into_response()
}
