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
use crate::common::auth::tokens::refresh_token::db::RefreshTokenRepository;
use crate::common::auth::tokens::tokens_service::TokensService;
use crate::common::error::Error;
use crate::features::users::db::UserRepository;

pub fn router() -> Router<AppState> {
    Router::new().route("/refresh-tokens", post(refresh_tokens))
}

async fn refresh_tokens(State(state): State<AppState>, jar: CookieJar) -> Result<impl IntoResponse, Error> {
    let Some(token) = jar.get("refresh_token") else {
        return Ok((StatusCode::BAD_REQUEST, "No refresh token in cookies").into_response());
    };
    let token_repo = RefreshTokenRepository::new(state.db_pool.clone());
    let user_repo = UserRepository::new(state.db_pool.clone());

    let token_service = TokensService::new(token_repo, user_repo);
    let tokens = token_service.refresh_tokens(&token.value()).await?;

    let new_jar = jar
        .add(Cookie::new("access_token", "placeholder"))
        .add(Cookie::new("refresh_token", "placeholder"));

    (new_jar, (StatusCode::OK, "Token refreshed")).into_response()
}
