pub enum Error {
    Conflict,
    Failure,
    Forbidden,
    NotFound,
    Unauthorized,
    Validation,
}

// TODO: Add more errors
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound,
            sqlx::Error::Database(db_err) => {
                if db_err.code().as_deref() == Some("23505") {
                    Error::Conflict
                } else {
                    Error::Failure
                }
            }
            _ => Error::Failure,
        }
    }
}
