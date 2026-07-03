use chrono::{DateTime, Utc};
use sqlx::{Error, PgPool};
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
    async fn find_registration_nonce(
        &self,
        nonce: &str,
    ) -> Result<Option<RegistrationNonce>, sqlx::Error>;
    async fn save_registration_nonce(&self, nonce: &RegistrationNonce)
    -> Result<bool, sqlx::Error>;
    async fn delete_registration_nonce(&self, nonce: &str) -> Result<(), sqlx::Error>;
    async fn delete_returning_registration_nonce(
        &self,
        nonce: &str,
    ) -> Result<Option<RegistrationNonce>, sqlx::Error>;
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
    async fn find_registration_nonce(
        &self,
        email: &str,
    ) -> Result<Option<RegistrationNonce>, sqlx::Error> {
        let result = sqlx::query_as!(
            RegistrationNonce,
            "SELECT * FROM registration_nonces WHERE nonce = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn save_registration_nonce(
        &self,
        nonce: &RegistrationNonce,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO registration_nonces VALUES ($1, $2, $3)",
            nonce.nonce,
            nonce.email,
            nonce.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() == 1)
    }

    async fn delete_registration_nonce(&self, nonce: &str) -> Result<(), Error> {
        sqlx::query!("DELETE FROM registration_nonces WHERE nonce = $1", nonce)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_returning_registration_nonce(
        &self,
        nonce: &str,
    ) -> Result<Option<RegistrationNonce>, Error> {
        let result = sqlx::query_as!(
            RegistrationNonce,
            "DELETE FROM registration_nonces WHERE nonce = $1 RETURNING *",
            nonce
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }
}
