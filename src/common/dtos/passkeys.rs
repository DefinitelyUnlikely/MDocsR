use serde::Deserialize;
use webauthn_rs::prelude::RegisterPublicKeyCredential;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct AddPasskeyStartRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct AddPasskeyFinishRequest {
    pub name: Option<String>,
    #[serde(flatten)]
    pub credential: RegisterPublicKeyCredential,
}
