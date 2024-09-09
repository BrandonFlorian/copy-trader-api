use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    // #[error("Not found: {0}")]
    // NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    // #[error("Internal server error")]
    // InternalServerError,

    #[error("Postgrest error: {0}")]
    PostgrestError(String),

    #[error("Json parse error: {0}")]
    JsonParseError(String),

    #[error("Request error: {0}")]
    RequestError(String), // Add this line
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            // AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            // AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            AppError::PostgrestError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            AppError::JsonParseError(message) => (StatusCode::BAD_REQUEST, message),
            AppError::RequestError(message) => (StatusCode::BAD_REQUEST, message),
        };

        (status, error_message).into_response()
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::BadRequest(err.to_string())
    }
}