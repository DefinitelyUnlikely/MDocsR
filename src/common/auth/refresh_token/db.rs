use sqlx::PgPool;
use crate::common::auth::refresh_token::refresh_token::RefreshToken;

pub async fn find_refresh_token(value: &str, pool: &PgPool) -> Option<RefreshToken> {
    println!("Fetching refresh token with value {}", value);
    let token = sqlx::query_as!(
        RefreshToken,
        "SELECT * FROM refresh_tokens WHERE token = $1",
        value
    )
        .fetch_one(pool)
        .await;

    if token.is_err() {
        None
    } else {
        Some(token.unwrap())
    }
}