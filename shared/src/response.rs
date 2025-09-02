use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

/// Success response
#[derive(Serialize, ToSchema, ToResponse)]
#[schema(title = "SuccessResponse")]
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

impl<T, M> SuccessResponse<T, M>
where
    T: Serialize + ToSchema,
    M: Serialize + ToSchema,
{
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
            meta: None,
        }
    }

    pub fn with_meta(data: T, meta: M) -> Self {
        Self {
            success: true,
            data,
            meta: Some(meta),
        }
    }
}

impl<T, M> IntoResponse for SuccessResponse<T, M>
where
    T: Serialize + ToSchema,
    M: Serialize + ToSchema,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// RFC 9457 Problem Details (error response)
#[derive(Serialize, ToSchema, ToResponse)]
#[schema(title = "ErrorResponse")]
pub struct ErrorResponse {
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

impl ErrorResponse {
    pub fn new(status: StatusCode, title: impl Into<String>) -> Self {
        Self {
            success: false,
            r#type: None,
            title: title.into(),
            status: status.as_u16(),
            detail: None,
            instance: None,
        }
    }

    pub fn with_detail(
        status: StatusCode,
        title: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            success: false,
            r#type: None,
            title: title.into(),
            status: status.as_u16(),
            detail: Some(detail.into()),
            instance: None,
        }
    }

    pub fn with_type_and_detail(
        status: StatusCode,
        title: impl Into<String>,
        r#type: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            success: false,
            r#type: Some(r#type.into()),
            title: title.into(),
            status: status.as_u16(),
            detail: Some(detail.into()),
            instance: None,
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}
