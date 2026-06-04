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
    let token_data = decode::<Claims>(token, &secret, &Validation::new(Algorithm::HS256))?;
    Ok(token_data)
}
