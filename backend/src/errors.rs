use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    NotFound(String),
    Internal(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {msg}"),
            ApiError::NotFound(msg) => write!(f, "Not Found: {msg}"),
            ApiError::Internal(msg) => write!(f, "Internal Error: {msg}"),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (actix_web::http::StatusCode::BAD_REQUEST, msg),
            ApiError::NotFound(msg) => (actix_web::http::StatusCode::NOT_FOUND, msg),
            ApiError::Internal(msg) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        HttpResponse::build(status).json(serde_json::json!({ "error": message }))
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        log::error!("Database error: {e:?}");
        ApiError::Internal("database error".into())
    }
}
