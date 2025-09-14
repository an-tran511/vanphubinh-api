use std::sync::Arc;

use axum::debug_handler;
use axum::extract::Query;
use axum::{Json, extract::State};
use products_management::domain::product_category::ProductCategory;
use products_management::usecases::list_product_categories::{
    ListProductCategoriesInput, ListProductCategoriesUsecase,
};
use serde::Deserialize;
use shared::usecase::Usecase;
use shared::{common_types::PaginationMeta, error::AppError, response::SuccessResponse};
use utoipa::{IntoParams, ToSchema};

use crate::app_state::AppState;

/// Request query parameters for listing product categories
#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListProductCategoriesRequest {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/list_product_categories",
    summary = "List paginated product categories",
    tag = "Product Categories",
    params(ListProductCategoriesRequest),
    responses(
        (status = 200, description = "List of product categories", body = inline(SuccessResponse<Vec<ProductCategory>, PaginationMeta>)),
    ),
)]
pub async fn list_product_categories(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<ListProductCategoriesRequest>,
) -> Result<Json<SuccessResponse<Vec<ProductCategory>, PaginationMeta>>, AppError> {
    let usecase = ListProductCategoriesUsecase::new(app_state.pool.clone());

    let input: ListProductCategoriesInput = ListProductCategoriesInput {
        page: query.page,
        per_page: query.per_page,
    };

    let output = usecase.execute(input).await.map_err(AppError::from)?;

    Ok(Json(SuccessResponse::with_meta(
        output.product_categories,
        output.meta,
    )))
}
