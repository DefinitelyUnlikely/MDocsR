use crate::common::auth::tokens::refresh_token::db::RefreshTokenRepository;
use crate::common::auth::tokens::refresh_token::refresh_token::RefreshToken;
use crate::common::auth::tokens::token::jwt::create_jwt;
use crate::common::error::Error;
use crate::features::users::db::UserRepository;

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
pub struct TokensService {
    refresh_token_repo: RefreshTokenRepository,
    user_repo: UserRepository,
}

impl TokensService {
    pub fn new(refresh_token_repo: RefreshTokenRepository, user_repo: UserRepository) -> Self {
        TokensService {
            refresh_token_repo,
            user_repo,
        }
    }

    /// This function takes a refresh token value, ensures it exists in the repository,
    /// is still valid, deletes the old refresh token and then generates a TokensResponse
    /// that contains a new short-lived JWT and a new refresh token.
    pub async fn refresh_tokens(&self, ref_token_value: &str) -> Result<TokensResponse, Error> {
        let token_result = self
            .refresh_token_repo
            .find_refresh_token(ref_token_value)
            .await;

        let opt_token = match token_result {
            Ok(t) => t,
            Err(sqlx::error::Error::RowNotFound) => return Err(Error::Unauthorized),
            _ => return Err(Error::Failure),
        };

        let Some(token) = opt_token else {
            return Err(Error::Unauthorized);
        };

        let valid = self.consume_refresh_token(&token).await?;
        if !valid {
            return Err(Error::Unauthorized);
        }

        let jwt_result = create_jwt(&token.user_id);

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

        if let None = self.user_repo.fetch_user_by_id(&token.user_id).await? {
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
