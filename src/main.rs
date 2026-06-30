use crate::common::auth::config::AuthConfig;
use crate::common::auth::routes::auth_router;
use crate::db::{create_pool, migrate};
use axum::Router;
use sqlx::PgPool;
use std::env;

pub mod common;
mod db;
pub mod features;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub auth_config: AuthConfig,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    let migration_result = migrate(&pool).await;

    match migration_result {
        Ok(_) => {}
        Err(error) => {
            panic!("Failed to migrate from database: {}", error);
        }
    }

    let auth_config = AuthConfig::from_env();

    let auth_router = auth_router();
    let app = Router::new()
        .nest("/auth", auth_router)
        .with_state(AppState {
            db_pool: pool.clone(),
            auth_config,
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
