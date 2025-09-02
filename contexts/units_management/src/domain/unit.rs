use rust_decimal::Decimal;
use serde::Serialize;
use shared::common_types::{PaginationInput, PaginationMeta};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnitType {
    pub id: Uuid,
    pub name: String,
    pub base_unit_id: Uuid,
}

#[derive(Debug, FromRow, Serialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub unit_type_id: Option<Uuid>,
    pub factor: Decimal,
    pub is_active: bool,
}

#[async_trait::async_trait]
pub trait UnitRepository: Send + Sync {
    async fn find_all_paginated(
        &self,
        pool: &PgPool,
        pagination: PaginationInput,
    ) -> Result<(Vec<Unit>, PaginationMeta), sqlx::Error>;
}
