use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterPasskeyRequest {
    email: String,
}

pub struct RegisterPasskeyResponse {}

pub struct LoginRequest {}

pub struct LoginResponse {}