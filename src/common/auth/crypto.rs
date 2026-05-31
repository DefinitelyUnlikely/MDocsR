use bcrypt::{BcryptError, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
}

pub fn verify_password(password: &str, hash: &str) -> Result<String, BcryptError> {
}