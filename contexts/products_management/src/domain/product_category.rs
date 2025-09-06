use serde::Serialize;
use shared::common_types::{PaginationInput, PaginationMeta};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProductCategory {
    pub id: Uuid,
    pub name: String,
    pub parent_product_category_id: Option<Uuid>,
}

#[async_trait::async_trait]
pub trait ProductCategoryRepository: Send + Sync {
    async fn find_all_paginated(
        &self,
        pool: &PgPool,
        pagination: PaginationInput,
    ) -> Result<(Vec<ProductCategory>, PaginationMeta), sqlx::Error>;

    async fn create(
        &self,
        pool: &PgPool,
        category: ProductCategory,
    ) -> Result<ProductCategory, sqlx::Error>;
}
