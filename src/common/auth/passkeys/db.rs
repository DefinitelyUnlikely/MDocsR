#[allow(async_fn_in_trait)]
pub trait PasskeyRepository: Send + Sync {
    async fn save_passkey();
    async fn load_passkeys_by_user_id();
}
