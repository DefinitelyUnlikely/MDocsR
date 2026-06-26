use std::env;

#[derive(Clone, Debug)]
pub struct AuthConfig {
    pub jwt_key: String,
    pub jwt_audience: String,
    pub jwt_issuer: String,
    pub jwt_expiration_seconds: u64,
}

impl AuthConfig {
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
