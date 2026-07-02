use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUserPayload {
    email: String,
}