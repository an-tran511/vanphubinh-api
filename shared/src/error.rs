use crate::response::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal(anyhow::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(msg) => {
                ErrorResponse::with_detail(StatusCode::NOT_FOUND, "Not Found", msg).into_response()
            }

            AppError::BadRequest(msg) => {
                ErrorResponse::with_detail(StatusCode::BAD_REQUEST, "Bad Request", msg)
                    .into_response()
            }

            AppError::Unauthorized => {
                ErrorResponse::new(StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            }

            AppError::Forbidden => {
                ErrorResponse::new(StatusCode::FORBIDDEN, "Forbidden").into_response()
            }

            AppError::Internal(err) => ErrorResponse::with_detail(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                err.to_string(),
            )
            .into_response(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UsecaseError {
    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Business rule violation: {0}")]
    BusinessRule(String),

    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl From<UsecaseError> for AppError {
    fn from(err: UsecaseError) -> Self {
        match err {
            UsecaseError::Validation(msg) => {
                error!("Validation error: {}", msg);
                AppError::BadRequest(msg)
            }
            UsecaseError::BusinessRule(msg) => {
                error!("Business rule violation: {}", msg);
                AppError::BadRequest(msg)
            }
            UsecaseError::Sqlx(sqlx_err) => {
                error!("Database error: {:?}", sqlx_err);
                AppError::Internal(sqlx_err.into())
            }
        }
    }
}
