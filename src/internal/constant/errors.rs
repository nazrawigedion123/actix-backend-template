// src/internal/constant/errors.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal database tracking anomaly encountered")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Requested resource for entity could not be verified")]
    NotFound,

    #[error("Validation failed: {0}")]
    ValidationError(String),
}

// Implement the Actix Web Response trait so errors turn into clean HTTP status codes later
impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::DatabaseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}
