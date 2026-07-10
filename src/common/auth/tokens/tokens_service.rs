use crate::common::auth::config::JwtConfig;
use crate::common::auth::tokens::refresh::refresh_token::RefreshToken;
use crate::common::auth::tokens::refresh::repository::RefreshTokenRepository;
use crate::common::auth::tokens::token::jwt::create_jwt;
use crate::common::error::Error;
use crate::features::users::repository::UserRepository;

pub struct TokensResponse {
    pub refresh_token_value: String,
    pub jwt_token: String,
}

impl TokensResponse {
    fn new(refresh_token_value: String, jwt_token: String) -> Self {
        TokensResponse {
            refresh_token_value,
            jwt_token,
        }
    }
}

pub struct TokensService<R, U> {
    refresh_token_repo: R,
    user_repo: U,
    auth_config: JwtConfig,
}

impl<R, U> TokensService<R, U>
where
    R: RefreshTokenRepository,
    U: UserRepository,
{
    pub fn new(refresh_token_repo: R, user_repo: U, auth_config: JwtConfig) -> Self {
        TokensService {
            refresh_token_repo,
            user_repo,
            auth_config,
        }
    }

    /// This function takes a refresh token value, ensures it exists in the repository,
    /// is still valid, deletes the old refresh token and then generates a TokensResponse
    /// that contains a new short-lived JWT and a new refresh token.
    pub async fn refresh_tokens(&self, ref_token_value: &str) -> Result<TokensResponse, Error> {
        let opt_token = self
            .refresh_token_repo
            .find_refresh_token(ref_token_value)
            .await?;

        let Some(token) = opt_token else {
            return Err(Error::Unauthorized);
        };

        let valid = self.consume_refresh_token(&token).await?;
        if !valid {
            return Err(Error::Unauthorized);
        }

        let jwt_result = create_jwt(&token.user_id, &self.auth_config);

        let jwt_token = match jwt_result {
            Ok(j) => j,
            Err(_err) => return Err(Error::Failure),
        };
        let refresh_token = RefreshToken::new(token.user_id);
        self.refresh_token_repo
            .save_refresh_token(&refresh_token)
            .await?;

        Ok(TokensResponse::new(refresh_token.token, jwt_token))
    }

    /// Validates a borrowed refresh token and remove it from the database.
    /// Returns a Result<bool, Error> to indicate if the token was valid or not.
    async fn consume_refresh_token(&self, token: &RefreshToken) -> Result<bool, Error> {
        if token.is_expired() {
            return Ok(false);
        }

        if self
            .user_repo
            .fetch_user_by_id(&token.user_id)
            .await?
            .is_none()
        {
            // TODO: Trace this somewhat unusual case
            return Ok(false);
        }

        let delete_affected = self
            .refresh_token_repo
            .delete_refresh_token(&token.token)
            .await?;

        if delete_affected == 0 {
            println!(
                "Log this! A token was not deleted even thought it should have existed just prior"
            )
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::users::user::User;
    use chrono::{Duration, Utc};
    use std::sync::Mutex;

    struct FakeUserRepository {
        users: Mutex<Vec<User>>,
    }

    impl UserRepository for FakeUserRepository {
        async fn save_user(&self, _user: User) -> Result<(), Error> {
            unimplemented!()
        }

        async fn fetch_user_by_id(&self, id: &str) -> Result<Option<User>, Error> {
            let users = self.users.lock().unwrap();
            Ok(users.iter().find(|u| u.id == id).cloned())
        }

        async fn fetch_user_by_email(&self, _email: &str) -> Result<Option<User>, Error> {
            unimplemented!()
        }
    }

    struct FakeRefreshTokenRepository {
        tokens: Mutex<Vec<RefreshToken>>,
    }

    impl RefreshTokenRepository for FakeRefreshTokenRepository {
        async fn save_refresh_token(&self, token: &RefreshToken) -> Result<bool, sqlx::Error> {
            let mut tokens = self.tokens.lock().unwrap();
            tokens.push(token.clone());
            Ok(true)
        }

        async fn find_refresh_token(
            &self,
            value: &str,
        ) -> Result<Option<RefreshToken>, sqlx::Error> {
            let tokens = self.tokens.lock().unwrap();
            Ok(tokens.iter().find(|t| t.token == value).cloned())
        }

        async fn delete_refresh_token(&self, value: &str) -> Result<u64, sqlx::Error> {
            let mut tokens = self.tokens.lock().unwrap();
            let initial_len = tokens.len();
            tokens.retain(|t| t.token != value);
            Ok((initial_len - tokens.len()) as u64)
        }
    }

    fn get_test_auth_config() -> JwtConfig {
        JwtConfig {
            jwt_key: "test-secret-key-for-jwt-signing".to_string(),
            jwt_audience: "test-audience".to_string(),
            jwt_issuer: "test-issuer".to_string(),
            jwt_expiration_seconds: 900,
        }
    }

    #[tokio::test]
    async fn test_refresh_tokens_success() {
        let auth_config = get_test_auth_config();
        let user = User::new("test@example.com".to_string());
        let token = RefreshToken::new(user.id.clone());

        let user_repo = FakeUserRepository {
            users: Mutex::new(vec![user.clone()]),
        };
        let refresh_token_repo = FakeRefreshTokenRepository {
            tokens: Mutex::new(vec![token.clone()]),
        };

        let service = TokensService::new(refresh_token_repo, user_repo, auth_config);
        let response = service.refresh_tokens(&token.token).await.unwrap();

        assert!(!response.jwt_token.is_empty());
        assert_ne!(response.refresh_token_value, token.token);
    }

    #[tokio::test]
    async fn test_refresh_tokens_expired() {
        let auth_config = get_test_auth_config();
        let user = User::new("test@example.com".to_string());
        let mut token = RefreshToken::new(user.id.clone());
        token.expires = Utc::now() - Duration::days(1); // Set to past

        let user_repo = FakeUserRepository {
            users: Mutex::new(vec![user.clone()]),
        };
        let refresh_token_repo = FakeRefreshTokenRepository {
            tokens: Mutex::new(vec![token.clone()]),
        };

        let service = TokensService::new(refresh_token_repo, user_repo, auth_config);
        let result = service.refresh_tokens(&token.token).await;

        assert!(matches!(result, Err(Error::Unauthorized)));
    }

    #[tokio::test]
    async fn test_refresh_tokens_not_found() {
        let auth_config = get_test_auth_config();
        let user = User::new("test@example.com".to_string());

        let user_repo = FakeUserRepository {
            users: Mutex::new(vec![user.clone()]),
        };
        let refresh_token_repo = FakeRefreshTokenRepository {
            tokens: Mutex::new(vec![]),
        };

        let service = TokensService::new(refresh_token_repo, user_repo, auth_config);
        let result = service.refresh_tokens("nonexistent_token").await;

        assert!(matches!(result, Err(Error::Unauthorized)));
    }

    #[tokio::test]
    async fn test_refresh_tokens_user_not_found() {
        let auth_config = get_test_auth_config();
        let token = RefreshToken::new("nonexistent_user".to_string());

        let user_repo = FakeUserRepository {
            users: Mutex::new(vec![]),
        };
        let refresh_token_repo = FakeRefreshTokenRepository {
            tokens: Mutex::new(vec![token.clone()]),
        };

        let service = TokensService::new(refresh_token_repo, user_repo, auth_config);
        let result = service.refresh_tokens(&token.token).await;

        assert!(matches!(result, Err(Error::Unauthorized)));
    }
}
