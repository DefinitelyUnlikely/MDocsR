use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterPasskeyRequest {
    pub(crate) email: String,
}

pub struct RegisterPasskeyResponse {}

pub struct LoginRequest {}

pub struct LoginResponse {}