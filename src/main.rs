pub mod common;
pub mod features;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
}
