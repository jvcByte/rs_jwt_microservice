use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)] // Unauthorized is part of the public API surface, used by consumers of this template
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    NotFound(String),
    InternalError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ApiError::BadRequest(m) => m,
            ApiError::Unauthorized(m) => m,
            ApiError::Forbidden(m) => m,
            ApiError::Conflict(m) => m,
            ApiError::NotFound(m) => m,
            ApiError::InternalError(m) => m,
        };
        write!(f, "{}", msg)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({ "error": self.to_string() }))
    }
}
