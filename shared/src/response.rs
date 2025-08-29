use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

/// Success response
#[derive(Serialize, ToSchema)]
pub struct SuccessResponse<T, M>
where
    T: Serialize + ToSchema,
    M: Serialize + ToSchema,
{
    pub success: bool,
    pub data: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<M>,
}

/// RFC 9457 Problem Details (error)
#[derive(Serialize, ToSchema)]
pub struct ProblemDetails {
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    pub title: String,
    pub status: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

/// Unified API response
#[derive(ToSchema)]
pub enum ApiResponse<T, M>
where
    T: Serialize + ToSchema,
    M: Serialize + ToSchema,
{
    Success(SuccessResponse<T, M>),
    Error(ProblemDetails),
}

impl<T, M> ApiResponse<T, M>
where
    T: Serialize + ToSchema,
    M: Serialize + ToSchema,
{
    pub fn success(data: T) -> Self {
        ApiResponse::Success(SuccessResponse {
            success: true,
            data,
            meta: None,
        })
    }

    pub fn with_meta(data: T, meta: M) -> Self {
        ApiResponse::Success(SuccessResponse {
            success: true,
            data,
            meta: Some(meta),
        })
    }

    pub fn error(status: StatusCode, title: impl Into<String>) -> Self {
        ApiResponse::Error(ProblemDetails {
            success: false,
            r#type: None,
            title: title.into(),
            status: status.as_u16(),
            detail: None,
            instance: None,
        })
    }
}

impl<T, M> IntoResponse for ApiResponse<T, M>
where
    T: Serialize + ToSchema,
    M: Serialize + ToSchema,
{
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Success(success) => (StatusCode::OK, Json(success)).into_response(),
            ApiResponse::Error(problem) => {
                let status = StatusCode::from_u16(problem.status)
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                (status, Json(problem)).into_response()
            }
        }
    }
}
