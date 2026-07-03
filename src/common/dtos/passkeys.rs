use serde::Deserialize;
use webauthn_rs::prelude::RegisterPublicKeyCredential;

#[derive(Deserialize)]
pub struct RegisterPasskeyRequest {
    pub(crate) email: String,
}

pub struct RegisterPasskeyResponse {}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: Option<String>,
}

pub struct LoginResponse {}

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
