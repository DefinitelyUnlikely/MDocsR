use axum_session::{SessionConfig, SessionLayer, SessionNullPool, SessionStore};
use chrono::TimeDelta;
use cookie::{Key, SameSite};

pub async fn build_session_layer() -> SessionLayer<SessionNullPool> {
    let key = Key::generate();

    let session_config = SessionConfig::default()
        .with_session_name("mdocsr_session")
        .with_key(key)
        .with_lifetime(TimeDelta::days(1))
        .with_http_only(true)
        .with_cookie_same_site(SameSite::Lax);

    let session_store = SessionStore::<SessionNullPool>::new(None, session_config)
        .await
        .expect("Failed to initialize session store");

    SessionLayer::new(session_store)
}