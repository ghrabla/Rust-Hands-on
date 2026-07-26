use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::app::AuthError;

#[derive(Debug)]
pub enum AppError {
    Unauthorized,
    Conflict(String),
    #[allow(dead_code)]
    BadRequest(String),
    Internal,
}

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::EmailTaken => AppError::Conflict("email already registered".to_string()),
            AuthError::InvalidCredentials => AppError::Unauthorized,
            AuthError::Internal => AppError::Internal,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "invalid email or password".to_string()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
