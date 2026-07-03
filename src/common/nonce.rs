use chrono::{DateTime, Utc};
use sqlx::PgPool;
use textnonce::TextNonce;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct RegistrationNonce {
    pub nonce: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl RegistrationNonce {
    pub fn new(email: String) -> RegistrationNonce {
        Self {
            nonce: TextNonce::new().to_string(),
            email,
            created_at: Utc::now(),
        }
    }
}

//-----------------
// NONCE DATABASE
//-----------------
#[allow(async_fn_in_trait)]
pub trait RegistrationNonceRepository: Send + Sync {
    async fn find_nonce(&self, nonce: &str) -> Result<Option<RegistrationNonce>, sqlx::Error>;
}

pub struct PostgresRegistrationNonceRepository {
    pool: PgPool,
}

impl PostgresRegistrationNonceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl RegistrationNonceRepository for PostgresRegistrationNonceRepository {
    async fn find_nonce(&self, email: &str) -> Result<Option<RegistrationNonce>, sqlx::Error> {
        let result: Option<RegistrationNonce> = sqlx::query_as!(RegistrationNonce, "SELECT * FROM registration_nonces WHERE nonce = $1", email);
    }
}