use crate::AppState;
use crate::common::error::Error;
use crate::common::nonce::{PostgresRegistrationNonceRepository, RegistrationNonceRepository};
use crate::features::users::db::{PostgresUserRepository, UserRepository};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_session::{Session, SessionNullPool};
use uuid::Uuid;
use webauthn_rs::prelude::*;

pub async fn register_start(
    State(state): State<AppState>,
    session: Session<SessionNullPool>,
    Path(nonce): Path<String>,
) -> Result<impl IntoResponse, Error> {
    // start by doing everything in the handler
    // and we can separate the concerns afterward.

    // Ensure the session is clean at the start by 
    // removing any stale session that might have existed. 
    session.remove("reg_state");
    let nonce_repo = PostgresRegistrationNonceRepository::new(state.db_pool.clone());
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());

    let Some(nonce) = nonce_repo.find_registration_nonce(&nonce).await? else {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    };
    nonce_repo.delete_registration_nonce(&nonce.nonce).await?;

    // This should not happen if we do the registration flow correctly
    // (User types in email, if they exist they get an email without the link)
    // But we may as well be safe rather than sorry I guess.
    if user_repo.fetch_user_by_email(&nonce.email).await?.is_some() {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    }

    let user_id = Uuid::new_v4();
    let res = match state.webauthn.start_passkey_registration(
        user_id,
        &nonce.email,
        &nonce.email,
        None,
    ) {
        Ok((ccr, reg_state)) => {
            session.set("reg_state", (nonce.email, user_id, reg_state));
            Json(ccr).into_response()
        }
        Err(_) => {
            return Err(Error::Failure);
        }
    };
    
    Ok(res)
}

pub async fn register_finish(
    State(state): State<AppState>,
    session: Session<SessionNullPool>,
    Json(reg_credential): Json<RegisterPublicKeyCredential>,
) -> Result<impl IntoResponse, Error> {
    let Some((email, user_id, reg_state)): Option<(String, Uuid, PasskeyRegistration)> =
        session.get("reg_state")
    else {
        return Ok((
            StatusCode::BAD_REQUEST,
            "Missing or expired registration session",
        )
            .into_response());
    };

    session.remove("reg_state");

    let passkey = match state
        .webauthn
        .finish_passkey_registration(&reg_credential, &reg_state)
    {
        Ok(passkey) => passkey,
        Err(_) => return Err(Error::Failure),
    };

    // TODO: Save user and passkey to database, consume registration nonce, and generate auth tokens.
    let _ = (email, user_id, passkey);
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());
    


    Ok(StatusCode::OK.into_response())
}

pub async fn login_start() -> impl IntoResponse {
    "Pong".to_string()
}
pub async fn login_finish() -> impl IntoResponse {
    "Pong".to_string()
}

pub async fn add_passkey_start() -> impl IntoResponse {
    "Pong".to_string()
}
pub async fn add_passkey_finish() -> impl IntoResponse {
    "Pong".to_string()
}
