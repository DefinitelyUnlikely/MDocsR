use serde::Deserialize;

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
