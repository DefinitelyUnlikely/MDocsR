use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};

use crate::AppState;
use crate::common::auth::tokens::token::jwt::decode_jwt;

/// Represents the authenticated caller extracted from a valid JWT.
///
/// Use as a handler parameter on any route that requires authentication:
/// ```rust
/// async fn my_handler(user: AuthenticatedUser, ...) -> impl IntoResponse { ... }
/// ```
///
/// Reads the `access_token` HttpOnly cookie and validates the JWT signature,
/// expiry, audience, and issuer using your [`AuthConfig`].
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
}

pub struct AuthError;

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| AuthError)?;

        let token = jar.get("access_token").ok_or(AuthError)?;

        let token_data = decode_jwt(token.value(), &state.auth_config).map_err(|_| AuthError)?;

        Ok(AuthenticatedUser {
            user_id: token_data.claims.sub,
        })
    }
}

pub fn build_access_cookie(token: String) -> Cookie<'static> {
    Cookie::build(("access_token", token))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .build()
}

pub fn build_refresh_cookie(token: String) -> Cookie<'static> {
    Cookie::build(("refresh", token))
        .path("/auth/refresh-tokens") // Scope refresh cookie to the refresh endpoint
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .build()
}
