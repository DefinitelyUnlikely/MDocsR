use std::env;
use webauthn_rs::prelude::Url;
use webauthn_rs::{Webauthn, WebauthnBuilder};

#[derive(Clone, Debug)]
pub struct JwtConfig {
    pub jwt_key: String,
    pub jwt_audience: String,
    pub jwt_issuer: String,
    pub jwt_expiration_seconds: u64,
}

/// A struct holding the required configurations for JWT workflows.
impl JwtConfig {
    pub fn from_env() -> Self {
        let jwt_key = env::var("JWT_KEY").expect("JWT_KEY must be set");
        let jwt_audience = env::var("JWT_AUDIENCE").expect("JWT_AUDIENCE must be set");
        let jwt_issuer = env::var("JWT_ISS").expect("JWT_ISS must be set");
        let jwt_expiration_seconds = env::var("EXPIRATION_IN_SECONDS")
            .unwrap_or_else(|_| "900".to_string())
            .parse()
            .expect("EXPIRATION_IN_SECONDS must be a valid u64");

        Self {
            jwt_key,
            jwt_audience,
            jwt_issuer,
            jwt_expiration_seconds,
        }
    }
}

/// A struct holding the required configurations for passkeys
#[derive(Clone, Debug)]
pub struct WebauthnConfig {
    pub rp_id: String,
    pub rp_origin_str: String,
    pub rp_origin: Url,
    pub rp_name: String,
}

impl WebauthnConfig {
    pub fn from_env() -> Self {
        let rp_id = env::var("WEBAUTHN_RP_ID").unwrap_or_else(|_| "localhost".to_string());
        let rp_origin_str =
            env::var("WEBAUTHN_RP_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let rp_origin = Url::parse(&rp_origin_str).expect("Invalid WEBAUTHN_RP_ORIGIN URL");
        let rp_name = env::var("WEBAUTHN_RP_NAME").unwrap_or_else(|_| "MDocsR".to_string());

        Self {
            rp_id,
            rp_origin_str,
            rp_origin,
            rp_name,
        }
    }
}
