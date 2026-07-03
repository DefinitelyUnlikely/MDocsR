use rand::Rng;
use textnonce::TextNonce;

pub fn nonce_generator() -> String {
    TextNonce::new().to_string()
}