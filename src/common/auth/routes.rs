use crate::AppState;
use crate::common::auth::passkeys::handlers::{
    add_passkey_finish, add_passkey_start, login_finish, login_start, register_finish,
    register_start,
};
use crate::common::auth::tokens::refresh::repository::PostgresRefreshTokenRepository;
use crate::common::auth::tokens::tokens_service::TokensService;
use crate::common::cookies::{build_access_cookie, build_refresh_cookie};
use crate::common::error::Error;
use crate::features::users::db::PostgresUserRepository;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use axum_extra::extract::CookieJar;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/refresh-tokens", post(refresh_tokens))
        .route("/ping", get(auth_router_ping))
        .route("/register/start/:nonce", post(register_start))
        .route("/register/finish", post(register_finish))
        .route("/login/start", post(login_start))
        .route("/login/finish", post(login_finish))
        .route("/passkeys/add/start", post(add_passkey_start))
        .route("/passkeys/add/finish", post(add_passkey_finish))
}

async fn auth_router_ping() -> impl IntoResponse {
    "Pong".to_string()
}

async fn refresh_tokens(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, Error> {
    let Some(token) = jar.get("refresh") else {
        return Ok((StatusCode::BAD_REQUEST, "No refresh token in cookies").into_response());
    };
    let token_repo = PostgresRefreshTokenRepository::new(state.db_pool.clone());
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());

    let token_service = TokensService::new(token_repo, user_repo, state.auth_config.clone());
    let tokens = token_service.refresh_tokens(token.value()).await?;

    let new_jar = jar
        .add(build_access_cookie(tokens.jwt_token))
        .add(build_refresh_cookie(tokens.refresh_token_value));

    Ok((new_jar, StatusCode::OK).into_response())
}
