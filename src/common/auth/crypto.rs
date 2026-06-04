use bcrypt::{BcryptError, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    bcrypt::hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    bcrypt::verify(password, hash)
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "my_super_secure_password";
        let hash = hash_password(password).expect("Should successfully hash password");

        // Assert that verify returns true for the correct password
        assert!(verify_password(password, &hash).expect("Should verify"));

        // Assert that verify returns false for an incorrect password
        assert!(!verify_password("wrong_password", &hash).expect("Should fail verify"));
    }
}