use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::CookieJar;

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

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::FromRequestParts;
    use axum::http::{Request, header};
    use sqlx::PgPool;

    use crate::common::auth::config::AuthConfig;
    use crate::common::auth::tokens::token::jwt::create_jwt;

    /// Builds a minimal AppState. The DB pool is never actually touched by the
    /// extractor — it only needs auth_config — so connect_lazy is safe here.
    fn test_state() -> AppState {
        AppState {
            db_pool: PgPool::connect_lazy("postgres://unused").unwrap(),
            auth_config: AuthConfig {
                jwt_key: "test-secret-that-is-long-enough".to_string(),
                jwt_audience: "test-audience".to_string(),
                jwt_issuer: "test-issuer".to_string(),
                jwt_expiration_seconds: 900,
            },
        }
    }

    /// Splits the request into parts and drives the extractor directly.
    /// This is the key pattern: no server, no HTTP round-trip needed.
    async fn extract(
        request: Request<()>,
        state: &AppState,
    ) -> Result<AuthenticatedUser, AuthError> {
        let (mut parts, _body) = request.into_parts();
        AuthenticatedUser::from_request_parts(&mut parts, state).await
    }

    #[tokio::test]
    async fn returns_401_when_access_token_cookie_is_missing() {
        let state = test_state();
        let request = Request::builder().body(()).unwrap();

        let result = extract(request, &state).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn returns_401_when_jwt_is_invalid() {
        let state = test_state();
        let request = Request::builder()
            .header(header::COOKIE, "access_token=this.is.not.a.valid.jwt")
            .body(())
            .unwrap();

        let result = extract(request, &state).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn returns_authenticated_user_when_jwt_is_valid() {
        let state = test_state();
        let user_id = "user-abc-123";

        // Create a real JWT signed with the same key/config the extractor will validate against
        let jwt = create_jwt(user_id, &state.auth_config).expect("Failed to create test JWT");

        let request = Request::builder()
            .header(header::COOKIE, format!("access_token={jwt}"))
            .body(())
            .unwrap();

        let result = extract(request, &state).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().user_id, user_id);
    }
}
