use axum::http::StatusCode;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub code: StatusCode,
    pub message: String,
}

impl Error {
    pub fn new(code: StatusCode, message: &str) -> Self {
        let error = Self {
            code,
            message: message.to_string(),
        };
        tracing::error!("{:#?}", error);
        error
    }

    pub fn internal_server(message: &str) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn not_found(message: &str) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn unauthorized(message: &str) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn bad_request(message: &str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for Error {}
