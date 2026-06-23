use crate::db::create_pool;
use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use sqlx::PgPool;
use std::env;

pub mod common;
mod db;
pub mod features;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    let app = Router::new()
        .route("/token/refresh", get(refresh_token))
        .with_state(AppState { db_pool: pool });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn refresh_token(jar: CookieJar) -> impl IntoResponse {
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
