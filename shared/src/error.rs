use crate::response::ProblemDetails;
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
        let problem_details = match self {
            AppError::NotFound(msg) => ProblemDetails {
                success: false,
                r#type: None,
                title: "Not Found".to_string(),
                status: StatusCode::NOT_FOUND.as_u16(),
                detail: Some(msg),
                instance: None,
            },

            AppError::BadRequest(msg) => ProblemDetails {
                success: false,
                r#type: None,
                title: "Bad Request".to_string(),
                status: StatusCode::BAD_REQUEST.as_u16(),
                detail: Some(msg),
                instance: None,
            },

            AppError::Unauthorized => ProblemDetails {
                success: false,
                r#type: None,
                title: "Unauthorized".to_string(),
                status: StatusCode::UNAUTHORIZED.as_u16(),
                detail: None,
                instance: None,
            },

            AppError::Forbidden => ProblemDetails {
                success: false,
                r#type: None,
                title: "Forbidden".to_string(),
                status: StatusCode::FORBIDDEN.as_u16(),
                detail: None,
                instance: None,
            },

            AppError::Internal(err) => ProblemDetails {
                success: false,
                r#type: None,
                title: "Internal Server Error".to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                detail: Some(err.to_string()),
                instance: None,
            },
        };

        let status = StatusCode::from_u16(problem_details.status)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, axum::Json(problem_details)).into_response()
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
