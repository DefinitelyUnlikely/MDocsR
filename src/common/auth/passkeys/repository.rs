use crate::common::error::Error;
use sqlx::PgPool;
use sqlx::types::Json;
use webauthn_rs::prelude::Passkey;

#[allow(async_fn_in_trait)]
pub trait PasskeyRepository: Send + Sync {
    async fn save_passkey(&self, user_id: &str, name: &str, passkey: &Passkey)
    -> Result<(), Error>;
    async fn load_passkeys_by_user_id(&self, user_id: &str) -> Result<Vec<Passkey>, Error>;
    async fn find_passkey_by_id(
        &self,
        id: &str,
    ) -> Result<Option<(String, String, Passkey)>, Error>;
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
            r#"
            INSERT INTO user_passkeys (id, user_id, name, credential_id, passkey)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET passkey = EXCLUDED.passkey, name = EXCLUDED.name
            "#,
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

    async fn find_passkey_by_id(
        &self,
        id: &str,
    ) -> Result<Option<(String, String, Passkey)>, Error> {
        let row = sqlx::query!(
            r#"SELECT user_id, name, passkey AS "passkey: Json<Passkey>" FROM user_passkeys WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| (r.user_id, r.name, r.passkey.0)))
    }
}
