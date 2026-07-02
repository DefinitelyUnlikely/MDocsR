use crate::common::auth::config::{JwtConfig, WebauthnConfig};
use crate::common::auth::routes::auth_router;
use crate::common::startup::{build_webauthn, build_session_layer};
use crate::db::{create_pool, migrate};
use axum::Router;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use webauthn_rs::{Webauthn};

pub mod common;
mod db;
pub mod features;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub auth_config: JwtConfig,
    pub webauthn: Arc<Webauthn>,
}

// TODO: Start breaking main up more, make startup file with related functions
// and run them from there perhaps?
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

    let auth_config = JwtConfig::from_env();
    
    let webauthn_config = WebauthnConfig::from_env();
    let webauthn = Arc::new(build_webauthn(
        &webauthn_config.rp_id,
        &webauthn_config.rp_origin,
        &webauthn_config.rp_name,
    ));
    
    let session_layer = build_session_layer()
        .await;

    let auth_router = auth_router();
    let app = Router::new()
        .nest("/auth", auth_router)
        .layer(session_layer)
        .with_state(AppState {
            db_pool: pool.clone(),
            auth_config,
            webauthn,
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
