use std::sync::Arc;

use axum::response::IntoResponse;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use shared::error::{AppError, UsecaseError};
use shared::response::ErrorResponse;
use shared::usecase::Usecase;
use units_management::usecases::create_unit::{
    CreateUnitInput, CreateUnitOutput, CreateUnitUsecase,
};

use crate::app_state::AppState;

#[derive(Deserialize, ToSchema)]
pub struct CreateUnitRequest {
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct CreateUnitResponse {
    pub id: String,
}

#[utoipa::path(
    post,
    path = "/create_unit",
        tag = "Units",
    request_body = CreateUnitRequest,
    responses(
        (status = 201, description = "Unit created successfully", body = CreateUnitResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn create_unit(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateUnitRequest>,
) -> Result<impl IntoResponse, AppError> {
    let usecase = CreateUnitUsecase::new(app_state.pool.clone());

    let input = CreateUnitInput { name: payload.name };

    match usecase.execute(input).await {
        Ok(CreateUnitOutput { unit }) => {
            let response = CreateUnitResponse {
                id: unit.id.to_string(),
            };
            Ok((axum::http::StatusCode::CREATED, Json(response)).into_response())
        }
        Err(e) => {
            let (status, title) = match &e {
                UsecaseError::Validation(_) => {
                    (axum::http::StatusCode::BAD_REQUEST, "Validation Error")
                }
                _ => (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                ),
            };
            let error_response = ErrorResponse::with_detail(status, title, e.to_string());
            Ok((status, Json(error_response)).into_response())
        }
    }
}
