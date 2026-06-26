use crate::AppState;
use crate::common::auth::tokens::refresh_token::db::RefreshTokenRepository;
use crate::common::auth::tokens::tokens_service::TokensService;
use crate::common::error::Error;
use crate::features::users::db::UserRepository;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};

pub fn auth_router() -> Router<AppState> {
    Router::new().route("/refresh-tokens", post(refresh_tokens))
}

async fn refresh_tokens(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, Error> {
    let Some(token) = jar.get("refresh_token") else {
        return Ok((StatusCode::BAD_REQUEST, "No refresh token in cookies").into_response());
    };
    let token_repo = RefreshTokenRepository::new(state.db_pool.clone());
    let user_repo = UserRepository::new(state.db_pool.clone());

    let token_service = TokensService::new(token_repo, user_repo);
    let tokens = token_service.refresh_tokens(&token.value()).await?;

    let access_cookie = Cookie::build(("access_token", tokens.jwt_token))
        .path("/")
        .http_only(true)
        .secure(true) // Forces HTTPS (strongly recommended)
        .same_site(SameSite::Strict) // Protects against CSRF
        .build();
    let refresh_cookie = Cookie::build(("refresh_token", tokens.refresh_token_value))
        .path("/")
        .http_only(true)
        .secure(true) // Forces HTTPS (strongly recommended)
        .same_site(SameSite::Strict) // Protects against CSRF
        .build();
    let new_jar = jar.add(access_cookie).add(refresh_cookie);

    Ok((new_jar, (StatusCode::OK, "Token refreshed")).into_response())
}
