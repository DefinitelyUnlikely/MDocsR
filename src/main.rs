use crate::db::create_pool;
use std::env;

pub mod common;
mod db;
pub mod features;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    create_pool(&database_url)
        .await
        .expect("Failed to create database pool");
}
