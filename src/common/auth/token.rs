use std::env;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, get_current_timestamp, TokenData};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    exp: u64,
    iss: String,
    sub: String,
}

fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        aud: String::from(env::var("JWT_AUDIENCE").expect("JWT_AUDIENCE not set!")),
        exp: get_current_timestamp() + 900, // TODO: Can also be a config for the duration we add
        iss: String::from(env::var("JWT_ISS").expect("JWT_ISS not set!")),
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
