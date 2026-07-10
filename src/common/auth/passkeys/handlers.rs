use crate::AppState;
use crate::common::auth::extractor::AuthenticatedUser;
use crate::common::auth::passkeys::repository::{PasskeyRepository, PostgresPasskeyRepository};
use crate::common::auth::tokens::refresh::refresh_token::RefreshToken;
use crate::common::auth::tokens::refresh::repository::{
    PostgresRefreshTokenRepository, RefreshTokenRepository,
};
use crate::common::auth::tokens::token::jwt::create_jwt;
use crate::common::cookies::{build_access_cookie, build_refresh_cookie};
use crate::common::dtos::passkeys::{
    AddPasskeyFinishRequest, AddPasskeyStartRequest, LoginRequest,
};
use crate::common::error::Error;
use crate::common::nonce::{PostgresRegistrationNonceRepository, RegistrationNonceRepository};
use crate::features::users::repository::{PostgresUserRepository, UserRepository};
use crate::features::users::user::User;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
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
    // I just need a good understanding of the flow etc. first.

    // Ensure the session is clean at the start by
    // removing any stale session that might have existed.
    session.remove("reg_state");
    let nonce_repo = PostgresRegistrationNonceRepository::new(state.db_pool.clone());
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());

    let Some(nonce) = nonce_repo
        .delete_returning_registration_nonce(&nonce)
        .await?
    else {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    };

    // This should not happen if we do the registration flow correctly
    // (User types in email, if they exist they get an email without the link)
    // But we may as well be safe rather than sorry I guess.
    if user_repo.fetch_user_by_email(&nonce.email).await?.is_some() {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    }

    let user_id = Uuid::new_v4();
    let res =
        match state
            .webauthn
            .start_passkey_registration(user_id, &nonce.email, &nonce.email, None)
        {
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

    let user_repo = PostgresUserRepository::new(state.db_pool.clone());
    user_repo
        .save_user(User::new_with_id(email, user_id.to_string()))
        .await?;

    let passkey_repo = PostgresPasskeyRepository::new(state.db_pool.clone());
    passkey_repo
        .save_passkey(&user_id.to_string(), "Default Passkey", &passkey)
        .await?;

    Ok(StatusCode::OK.into_response())
}

pub async fn login_start(
    State(state): State<AppState>,
    session: Session<SessionNullPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, Error> {
    session.remove("auth_state");

    let passkeys = if let Some(ref email) = payload.email {
        let user_repo = PostgresUserRepository::new(state.db_pool.clone());
        if let Ok(Some(user)) = user_repo.fetch_user_by_email(email).await {
            let passkey_repo = PostgresPasskeyRepository::new(state.db_pool.clone());
            passkey_repo
                .load_passkeys_by_user_id(&user.id)
                .await
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let (rcr, auth_state) = match state.webauthn.start_passkey_authentication(&passkeys) {
        Ok((rcr, auth_state)) => (rcr, auth_state),
        Err(_) => return Err(Error::Failure),
    };

    session.set("auth_state", auth_state);

    Ok(Json(rcr).into_response())
}

pub async fn login_finish(
    State(state): State<AppState>,
    session: Session<SessionNullPool>,
    jar: CookieJar,
    Json(auth_credential): Json<PublicKeyCredential>,
) -> Result<impl IntoResponse, Error> {
    let Some(auth_state): Option<PasskeyAuthentication> = session.get("auth_state") else {
        return Ok((StatusCode::BAD_REQUEST, "Missing or expired login session").into_response());
    };

    session.remove("auth_state");

    let auth_result = match state
        .webauthn
        .finish_passkey_authentication(&auth_credential, &auth_state)
    {
        Ok(res) => res,
        Err(_) => return Err(Error::Unauthorized),
    };

    let cred_id_hex = hex::encode(auth_result.cred_id().as_slice());
    let passkey_repo = PostgresPasskeyRepository::new(state.db_pool.clone());

    let Some((user_id, name, mut passkey)) = passkey_repo.find_passkey_by_id(&cred_id_hex).await?
    else {
        return Err(Error::Unauthorized);
    };

    if passkey.update_credential(&auth_result).unwrap_or(false) {
        passkey_repo.save_passkey(&user_id, &name, &passkey).await?;
    }

    let user_repo = PostgresUserRepository::new(state.db_pool.clone());
    let Some(user) = user_repo.fetch_user_by_id(&user_id).await? else {
        return Err(Error::Unauthorized);
    };

    let jwt_token = match create_jwt(&user.id, &state.auth_config) {
        Ok(j) => j,
        Err(_) => return Err(Error::Failure),
    };

    let refresh_token = RefreshToken::new(user.id.clone());
    let refresh_token_repo = PostgresRefreshTokenRepository::new(state.db_pool.clone());
    refresh_token_repo
        .save_refresh_token(&refresh_token)
        .await?;

    let new_jar = jar
        .add(build_access_cookie(jwt_token))
        .add(build_refresh_cookie(refresh_token.token));

    Ok((new_jar, StatusCode::OK).into_response())
}

pub async fn add_passkey_start(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    session: Session<SessionNullPool>,
    Json(payload): Json<AddPasskeyStartRequest>,
) -> Result<impl IntoResponse, Error> {
    session.remove("add_passkey_state");

    let user_repo = PostgresUserRepository::new(state.db_pool.clone());
    let Some(db_user) = user_repo.fetch_user_by_id(&user.user_id).await? else {
        return Err(Error::Unauthorized);
    };

    let user_id_uuid = match Uuid::parse_str(&user.user_id) {
        Ok(u) => u,
        Err(_) => return Err(Error::Unauthorized),
    };

    let passkey_repo = PostgresPasskeyRepository::new(state.db_pool.clone());
    let existing_passkeys = passkey_repo
        .load_passkeys_by_user_id(&user.user_id)
        .await
        .unwrap_or_default();

    let exclude_keys = if existing_passkeys.is_empty() {
        None
    } else {
        Some(
            existing_passkeys
                .iter()
                .map(|p| p.cred_id().clone())
                .collect(),
        )
    };

    let (ccr, reg_state) = match state.webauthn.start_passkey_registration(
        user_id_uuid,
        &db_user.email,
        &db_user.email,
        exclude_keys,
    ) {
        Ok((ccr, reg_state)) => (ccr, reg_state),
        Err(_) => return Err(Error::Failure),
    };

    let name = if payload.name.trim().is_empty() {
        "Passkey".to_string()
    } else {
        payload.name
    };

    session.set("add_passkey_state", (user.user_id, name, reg_state));

    Ok(Json(ccr).into_response())
}

pub async fn add_passkey_finish(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    session: Session<SessionNullPool>,
    Json(payload): Json<AddPasskeyFinishRequest>,
) -> Result<impl IntoResponse, Error> {
    let Some((session_user_id, session_name, reg_state)): Option<(
        String,
        String,
        PasskeyRegistration,
    )> = session.get("add_passkey_state") else {
        return Ok((
            StatusCode::BAD_REQUEST,
            "Missing or expired passkey addition session",
        )
            .into_response());
    };

    if session_user_id != user.user_id {
        return Err(Error::Unauthorized);
    }

    session.remove("add_passkey_state");

    let passkey = match state
        .webauthn
        .finish_passkey_registration(&payload.credential, &reg_state)
    {
        Ok(passkey) => passkey,
        Err(_) => return Err(Error::Failure),
    };

    let final_name = payload
        .name
        .filter(|n| !n.trim().is_empty())
        .unwrap_or(session_name);

    let passkey_repo = PostgresPasskeyRepository::new(state.db_pool.clone());
    passkey_repo
        .save_passkey(&user.user_id, &final_name, &passkey)
        .await?;

    Ok(StatusCode::OK.into_response())
}
