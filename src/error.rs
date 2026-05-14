use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("internal error")]
    Internal,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn public_code(&self) -> &'static str {
        match self {
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::Unauthorized => "UNAUTHORIZED",
            AppError::NotFound => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::Internal => "INTERNAL_ERROR",
        }
    }

    pub fn public_message(&self) -> String {
        match self {
            AppError::Internal => "an internal error occurred".to_string(),
            other => other.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let body = Json(json!({
            "error": {
                "code": self.public_code(),
                "message": self.public_message()
            }
        }));
        (status, body).into_response()
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(value: validator::ValidationErrors) -> Self {
        AppError::Validation(value.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!(?error, "database error");
        AppError::Internal
    }
}
