use async_trait::async_trait;
use derive_more::From;
use shared::{
    common_types::{PaginationInput, PaginationMeta},
    error::UsecaseError,
    usecase::Usecase,
};
use sqlx::PgPool;

use crate::{
    domain::product_category::{ProductCategory, ProductCategoryRepository},
    infra::repos::product_category_repo::ProductCategoryRepoImpl,
};

#[derive(From)]
pub struct ListProductCategoriesInput {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

pub struct ListProductCategoriesOutput {
    pub product_categories: Vec<ProductCategory>,
    pub meta: PaginationMeta,
}

pub struct ListProductCategoriesUsecase {
    pool: PgPool,
    product_category_repository: ProductCategoryRepoImpl,
}

impl ListProductCategoriesUsecase {
    pub fn new(pool: PgPool) -> Self {
        let product_category_repository = ProductCategoryRepoImpl;

        Self {
            pool,
            product_category_repository,
        }
    }
}

#[async_trait]
impl Usecase<ListProductCategoriesInput, ListProductCategoriesOutput, UsecaseError>
    for ListProductCategoriesUsecase
{
    async fn execute(
        &self,
        input: ListProductCategoriesInput,
    ) -> Result<ListProductCategoriesOutput, UsecaseError> {
        let (product_categories, meta) = self
            .product_category_repository
            .find_all_paginated(
                &self.pool,
                PaginationInput {
                    page: input.page,
                    per_page: input.per_page,
                },
            )
            .await?;

        Ok(ListProductCategoriesOutput {
            product_categories,
            meta,
        })
    }
}
