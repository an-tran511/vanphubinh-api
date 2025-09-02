use crate::domain::unit::{Unit, UnitRepository};
use shared::common_types::{PaginationInput, PaginationMeta};
use sqlx::PgPool;

pub struct UnitRepoImpl;

#[async_trait::async_trait]
impl UnitRepository for UnitRepoImpl {
    async fn find_all_paginated(
        &self,
        pool: &PgPool,
        pagination: PaginationInput,
    ) -> Result<(Vec<Unit>, PaginationMeta), sqlx::Error> {
        let page = pagination.page.unwrap_or(1);
        let page_size = pagination.per_page.unwrap_or(10);

        let offset = page.saturating_sub(1) * page_size;

        // Total count
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM units")
            .fetch_one(pool)
            .await?;

        // Paginated query
        let units =
            sqlx::query_as::<_, Unit>("SELECT * FROM units ORDER BY id ASC LIMIT $1 OFFSET $2")
                .bind(page_size)
                .bind(offset)
                .fetch_all(pool)
                .await?;

        Ok((
            units,
            PaginationMeta {
                page,
                per_page: page_size,
                total,
                total_pages: (total as f64 / page_size as f64).ceil() as i64,
            },
        ))
    }
}
