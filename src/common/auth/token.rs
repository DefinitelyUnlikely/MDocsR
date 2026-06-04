use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
    get_current_timestamp,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    exp: u64,
    iss: String,
    sub: String,
}

fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let seconds_to_expire: u64 = env::var("EXPIRATION_IN_SECONDS")
        .unwrap_or(String::from("900"))
        .parse()
        .unwrap_or(900);
    let claims = Claims {
        aud: env::var("JWT_AUDIENCE").expect("JWT_AUDIENCE not set!"),
        exp: get_current_timestamp() + seconds_to_expire,
        iss: env::var("JWT_ISS").expect("JWT_ISS not set!"),
        sub: user_id.to_string(),
    };

    let secret = EncodingKey::from_secret(env::var("JWT_KEY").expect("JWT_KEY not set!").as_ref());
    encode(&Header::default(), &claims, &secret)
}

fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = DecodingKey::from_secret(env::var("JWT_KEY").expect("JWT_KEY not set!").as_ref());
    let mut validation = Validation::new(Algorithm::HS256);

    let aud = env::var("JWT_AUDIENCE").expect("JWT_AUDIENCE not set!");
    validation.set_audience(&[aud]);

    let iss = env::var("JWT_ISS").expect("JWT_ISS not set!");
    validation.set_issuer(&[iss]);

    let token_data = decode::<Claims>(token, &secret, &validation)?;
    Ok(token_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    // Unsafe should be fine here, as its code only being run for testing.
    // and according to the docs set var is unsafe if you have multiple threads
    // on a none windows operating system. Should not be the case for tests.
    fn setup_env() {
        INIT.call_once(|| unsafe {
            env::set_var("JWT_KEY", "test-secret-key-for-jwt-signing");
            env::set_var("JWT_AUDIENCE", "test-audience");
            env::set_var("JWT_ISS", "test-issuer");
        });
    }

    #[test]
    fn test_create_and_decode_jwt() {
        setup_env();
        let user_id = "user-abc-123";

        let token = create_jwt(user_id).expect("Failed to create JWT");
        assert!(!token.is_empty());
        let token_data = decode_token(&token).expect("Failed to decode JWT");

        assert_eq!(token_data.claims.sub, user_id);
        assert_eq!(token_data.claims.aud, env::var("JWT_AUDIENCE").unwrap());
        assert_eq!(token_data.claims.iss, env::var("JWT_ISS").unwrap());
        assert!(token_data.claims.exp > get_current_timestamp());
    }
}
