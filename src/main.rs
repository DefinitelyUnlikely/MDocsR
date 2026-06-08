pub mod common;
pub mod features;
mod db;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
}
