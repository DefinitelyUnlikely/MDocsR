use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    exp: usize,
    iss: String,
    sub: String,
}

fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        aud: String::from("TODO: Make use of config"),
        exp: Utc::now().timestamp() as usize + 900, // TODO: Can also be a config for the duration we add
        iss: String::from("TODO: Make use of config for iss"),
        sub: user_id.to_string(),
    };

    let secret = EncodingKey::from_secret("super_secret_key".as_ref());
    encode(&Header::default(), &claims, &secret)
}