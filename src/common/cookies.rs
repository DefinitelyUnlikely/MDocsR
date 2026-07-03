use axum_extra::extract::cookie::{Cookie, SameSite};
use cookie::time::Duration;

pub fn build_access_cookie(token: String) -> Cookie<'static> {
    Cookie::build(("access_token", token))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(Duration::minutes(15))
        .build()
}

pub fn build_refresh_cookie(token: String) -> Cookie<'static> {
    Cookie::build(("refresh", token))
        .path("/auth/refresh-tokens") // Scope refresh cookie to the refresh endpoint
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(Duration::days(7))
        .build()
}
