use crate::common::auth::tokens::refresh_token::db::RefreshTokenRepository;
use crate::common::auth::tokens::refresh_token::refresh_token::RefreshToken;
use crate::common::auth::tokens::token::jwt;
use crate::common::error::Error;

struct TokensResponse {
    refresh_token_value: String,
    jwt_token: String,
}

impl TokensResponse {
    fn new(refresh_token_value: String, jwt_token: String) -> Self {
        TokensResponse { refresh_token_value, jwt_token }
    }
}
pub struct TokensService {
    refresh_token_repo: RefreshTokenRepository,
}

impl TokensService {
    pub fn new(refresh_token_repo: RefreshTokenRepository) -> Self {
        TokensService { refresh_token_repo }
    }

    pub async fn refresh_tokens(&self, ref_token_value: &str) -> Result<TokensResponse, Error> {
        let token_result = self.refresh_token_repo.find_refresh_token(ref_token_value).await;

        let opt_token = match token_result {
            Ok(t) => t,
            Err(sqlx::error::Error::RowNotFound) => return Err(Error::Unauthorized),
            _ => return Err(Error::Failure)
        };

        let Some(token) = opt_token else {
            return Err(Error::Unauthorized);
        };

        let valid = self.consume_refresh_token(&token).await?;

        let refresh_token = RefreshToken::new(token.user_id);
        let jwt_token = create_jwt();

    }

    /// Validates and consumes a refresh token value. Returns a Result<bool, Error>
    /// to indicate if the token was valid or not.
    async fn consume_refresh_token(&self, token: &RefreshToken) -> Result<bool, Error> {
    }

}