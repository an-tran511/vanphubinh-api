use std::sync::Arc;

use axum::response::IntoResponse;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use products_management::usecases::create_product_category::{
    CreateProductCategoryInput, CreateProductCategoryOutput, CreateProductCategoryUsecase,
};
use shared::error::{AppError, UsecaseError};
use shared::response::ErrorResponse;
use shared::usecase::Usecase;

use crate::app_state::AppState;

#[derive(Deserialize, ToSchema)]
pub struct CreateProductCategoryRequest {
    pub name: String,
    pub parent_product_category_id: Option<uuid::Uuid>,
}

#[derive(Serialize, ToSchema)]
pub struct CreateProductCategoryResponse {
    pub id: String,
}

#[utoipa::path(
    post,
    path = "/create_product_category",
    summary = "Create a new product category",
    description = "Create a new product category",
    tag = "Product Categories",
    request_body = CreateProductCategoryRequest,
    responses(
        (status = 201, description = "Product category created successfully", body = CreateProductCategoryResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn create_product_category(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateProductCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let usecase = CreateProductCategoryUsecase::new(app_state.pool.clone());

    let input = CreateProductCategoryInput {
        name: payload.name,
        parent_product_category_id: payload.parent_product_category_id,
    };

    match usecase.execute(input).await {
        Ok(CreateProductCategoryOutput { product_category }) => {
            let response = CreateProductCategoryResponse {
                id: product_category.id.to_string(),
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
