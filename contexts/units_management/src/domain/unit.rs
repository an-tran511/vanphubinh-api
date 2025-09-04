use serde::Serialize;
use shared::common_types::{PaginationInput, PaginationMeta};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub id: Uuid,
    pub name: String,
}

#[async_trait::async_trait]
pub trait UnitRepository: Send + Sync {
    async fn find_all_paginated(
        &self,
        pool: &PgPool,
        pagination: PaginationInput,
    ) -> Result<(Vec<Unit>, PaginationMeta), sqlx::Error>;
}
