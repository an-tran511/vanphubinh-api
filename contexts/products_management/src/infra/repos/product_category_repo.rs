use crate::domain::product_category::{ProductCategory, ProductCategoryRepository};
use shared::common_types::{PaginationInput, PaginationMeta};
use sqlx::PgPool;

pub struct ProductCategoryRepoImpl;

#[async_trait::async_trait]
impl ProductCategoryRepository for ProductCategoryRepoImpl {
    async fn find_all_paginated(
        &self,
        pool: &PgPool,
        pagination: PaginationInput,
    ) -> Result<(Vec<ProductCategory>, PaginationMeta), sqlx::Error> {
        let page = pagination.page.unwrap_or(1);
        let page_size = pagination.per_page.unwrap_or(10);

        let offset = page.saturating_sub(1) * page_size;

        // Total count
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM product_categories")
            .fetch_one(pool)
            .await?;

        // Paginated query
        let product_categories = sqlx::query_as::<_, ProductCategory>(
            "SELECT * FROM product_categories ORDER BY id ASC LIMIT $1 OFFSET $2",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((
            product_categories,
            PaginationMeta {
                page,
                per_page: page_size,
                total,
                total_pages: (total as f64 / page_size as f64).ceil() as i64,
            },
        ))
    }

    async fn create(
        &self,
        pool: &PgPool,
        unit: ProductCategory,
    ) -> Result<ProductCategory, sqlx::Error> {
        let product_category = sqlx::query_as::<_, ProductCategory>(
            "INSERT INTO product_categories (id, name) VALUES ($1, $2) RETURNING *",
        )
        .bind(unit.id)
        .bind(unit.name)
        .fetch_one(pool)
        .await?;

        Ok(product_category)
    }
}
