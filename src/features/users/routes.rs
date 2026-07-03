    use axum::{Json, Router};
    use axum::extract::State;
    use axum::response::IntoResponse;
    use axum::routing::post;
    use crate::AppState;
    use crate::common::error::Error;
    use crate::common::nonce::{PostgresRegistrationNonceRepository, RegistrationNonce, RegistrationNonceRepository};
    use crate::features::users::dtos::RegisterUserRequest;

    pub fn user_router() -> Router<AppState> {
        Router::new()
            .route("/register", post(register_user))
    }

    async fn register_user(
        State(state): State<AppState>,
        Json(payload): Json<RegisterUserRequest>,
    ) -> Result<impl IntoResponse, Error> {
        // For now, this handler will simply return the URL,
        // and we can start registering a passkey using that.
        // In the future, this will be replaced with instead generating
        // the link and sending it to the email specified.

        let nonce = RegistrationNonce::new(payload.email);
        let nonce_repo = PostgresRegistrationNonceRepository::new(state.db_pool.clone());
        let result = nonce_repo.save_registration_nonce(&nonce).await?;

        if result {
            Ok(Json(nonce.nonce))
        } else {
            Err(Error::Failure)
        }
    }