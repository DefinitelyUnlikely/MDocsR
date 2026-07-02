use crate::common::auth::config::JwtConfig;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
    get_current_timestamp,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub exp: u64,
    pub iss: String,
    pub sub: String,
}

pub fn create_jwt(
    user_id: &str,
    config: &JwtConfig,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        aud: config.jwt_audience.clone(),
        exp: get_current_timestamp() + config.jwt_expiration_seconds,
        iss: config.jwt_issuer.clone(),
        sub: user_id.to_string(),
    };

    let secret = EncodingKey::from_secret(config.jwt_key.as_bytes());
    encode(&Header::default(), &claims, &secret)
}

pub fn decode_jwt(
    token: &str,
    config: &JwtConfig,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = DecodingKey::from_secret(config.jwt_key.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);

    validation.set_audience(&[&config.jwt_audience]);
    validation.set_issuer(&[&config.jwt_issuer]);

    let token_data = decode::<Claims>(token, &secret, &validation)?;
    Ok(token_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_decode_jwt() {
        let config = JwtConfig {
            jwt_key: "test-secret-key-for-jwt-signing".to_string(),
            jwt_audience: "test-audience".to_string(),
            jwt_issuer: "test-issuer".to_string(),
            jwt_expiration_seconds: 900,
        };
        let user_id = "user-abc-123";

        let token = create_jwt(user_id, &config).expect("Failed to create JWT");
        assert!(!token.is_empty());
        let token_data = decode_jwt(&token, &config).expect("Failed to decode JWT");

        assert_eq!(token_data.claims.sub, user_id);
        assert_eq!(token_data.claims.aud, config.jwt_audience);
        assert_eq!(token_data.claims.iss, config.jwt_issuer);
        assert!(token_data.claims.exp > get_current_timestamp());
    }
}
