use sqlx::{query_as, PgPool};
use crate::common::error::Error;
use crate::features::users::user::User;

pub async fn fetch_user_by_id(id: String, pool: &PgPool, include_info: bool) -> Result<User, Error> {
    todo!()
}

async fn fetch_user_with_info(id: String, pool: &PgPool) -> Result<User, Error> {}

async fn fetch_user_without_info(id: String, pool: &PgPool) -> Result<User, Error> {
    
}