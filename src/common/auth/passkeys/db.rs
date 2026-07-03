use crate::common::error::Error;
use sqlx::PgPool;
use sqlx::types::Json;
use webauthn_rs::prelude::Passkey;

#[allow(async_fn_in_trait)]
pub trait PasskeyRepository: Send + Sync {
    async fn save_passkey(&self, user_id: &str, name: &str, passkey: &Passkey)
    -> Result<(), Error>;
    async fn load_passkeys_by_user_id(&self, user_id: &str) -> Result<Vec<Passkey>, Error>;
}

pub struct PostgresPasskeyRepository {
    pool: PgPool,
}

impl PostgresPasskeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl PasskeyRepository for PostgresPasskeyRepository {
    async fn save_passkey(
        &self,
        user_id: &str,
        name: &str,
        passkey: &Passkey,
    ) -> Result<(), Error> {
        let cred_id_bytes = passkey.cred_id().as_slice();
        let id = hex::encode(cred_id_bytes);

        sqlx::query!(
            "
            INSERT INTO user_passkeys (id, user_id, name, credential_id, passkey)
            VALUES ($1, $2, $3, $4, $5)
            ",
            id,
            user_id,
            name,
            cred_id_bytes,
            Json(passkey) as _
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn load_passkeys_by_user_id(&self, user_id: &str) -> Result<Vec<Passkey>, Error> {
        let rows = sqlx::query!(
            r#"SELECT passkey AS "passkey: Json<Passkey>" FROM user_passkeys WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let passkeys = rows.into_iter().map(|row| row.passkey.0).collect();

        Ok(passkeys)
    }
}
