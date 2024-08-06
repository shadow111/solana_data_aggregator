use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Not found")]
    NotFound,
    #[error("Internal server error")]
    InternalError,
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub code:    u16,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            ApiError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(ErrorMessage {
            code:    status.as_u16(),
            message: error_message.to_string(),
        });

        (status, body).into_response()
    }
}
