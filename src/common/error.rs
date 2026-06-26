use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum Error {
    Conflict,
    Failure,
    Forbidden,
    NotFound,
    Unauthorized,
    Validation,
}

impl From<Error> for StatusCode {
    fn from(err: Error) -> StatusCode {
        match err {
            Error::Conflict => StatusCode::CONFLICT,
            Error::Failure => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Validation => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = StatusCode::from(self);
        status.into_response()
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
