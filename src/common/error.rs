use axum::http::StatusCode;

pub enum Error {
    Conflict,
    Failure,
    Forbidden,
    NotFound,
    Unauthorized,
    Validation,
}

impl Into<StatusCode> for Error {
    fn into(self) -> StatusCode {
        match self {
            Error::Conflict => StatusCode::CONFLICT,
            Error::Failure => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Validation => StatusCode::BAD_REQUEST,
        }
    }
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
