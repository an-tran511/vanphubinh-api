use std::sync::Arc;

use axum::debug_handler;
use axum::extract::Query;
use axum::{Json, extract::State};
use serde::Deserialize;
use shared::usecase::Usecase;
use shared::{common_types::PaginationMeta, error::AppError, response::SuccessResponse};
use units_management::domain::unit::Unit;
use units_management::usecases::list_units::{ListUnitsInput, ListUnitsUsecase};
use utoipa::{IntoParams, ToSchema};

use crate::app_state::AppState;

/// Request query parameters for listing units
#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListUnitsRequest {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/list_units",
    tag = "Units",
    params(ListUnitsRequest),
    responses(
        (status = 200, description = "List of units", body = inline(SuccessResponse<Vec<Unit>, PaginationMeta>)),
    ),
)]
pub async fn list_units(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<ListUnitsRequest>,
) -> Result<Json<SuccessResponse<Vec<Unit>, PaginationMeta>>, AppError> {
    let usecase = ListUnitsUsecase::new(app_state.pool.clone());

    let input: ListUnitsInput = ListUnitsInput {
        page: query.page,
        per_page: query.per_page,
    };

    let output = usecase.execute(input).await.map_err(AppError::from)?;

    Ok(Json(SuccessResponse::with_meta(output.units, output.meta)))
}
