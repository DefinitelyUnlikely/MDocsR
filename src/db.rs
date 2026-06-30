use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::Error> {
    // TODO: Make into transaction
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS users 
        (id VARCHAR(255) PRIMARY KEY UNIQUE NOT NULL,
        email VARCHAR(255) UNIQUE NOT NULL,
        password_hash VARCHAR(255) UNIQUE NOT NULL,
        email_verified BOOLEAN UNIQUE NOT NULL DEFAULT FALSE,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP);"
    ).execute(pool).await?;

    sqlx::query!("CREATE TABLE IF NOT EXISTS user_infos (
    id VARCHAR(255) PRIMARY KEY UNIQUE NOT NULL,
    user_id VARCHAR(255) UNIQUE NOT NULL REFERENCES users ON DELETE CASCADE,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    phone VARCHAR(50),
    country VARCHAR (100) NOT NULL,
    city VARCHAR (100) NOT NULL,
    state VARCHAR (100),
    address VARCHAR (255) NOT NULL,
    postal_code VARCHAR(20) NOT NULL
    );").execute(pool).await?;
    
    sqlx::query!("CREATE TABLE IF NOT EXISTS refresh_tokens (
    token VARCHAR(255) PRIMARY KEY UNIQUE NOT NULL,
    user_id VARCHAR(255) NOT NULL REFERENCES users ON DELETE CASCADE,
    expires TIMESTAMP WITH TIME ZONE NOT NULL);").execute(pool).await?;
    
    Ok(())
}
