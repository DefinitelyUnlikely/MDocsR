use crate::common::auth::tokens::refresh_token::db::RefreshTokenRepository;
use crate::common::error::Error;

pub struct TokensService {
    refresh_token_repo: RefreshTokenRepository,
}

impl TokensService {
    pub fn new(refresh_token_repo: RefreshTokenRepository) -> Self {
        TokensService { refresh_token_repo }
    }

    /// Validates and consumes a refresh token value. Returns a Result<bool, Error>
    /// to indicate if the token was valid or not.
    pub async fn consume_refresh_token(&self, value: String) -> Result<bool, Error> {
        let opt_token = self.refresh_token_repo.find_refresh_token(&value).await;
    }
}