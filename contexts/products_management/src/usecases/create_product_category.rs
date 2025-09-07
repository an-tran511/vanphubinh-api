use async_trait::async_trait;
use derive_more::From;
use shared::error::UsecaseError;
use shared::usecase::Usecase;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::product_category::{ProductCategory, ProductCategoryRepository},
    infra::repos::product_category_repo::ProductCategoryRepoImpl,
};

#[derive(From)]
pub struct CreateProductCategoryInput {
    pub name: String,
    pub parent_product_category_id: Option<Uuid>,
}

pub struct CreateProductCategoryOutput {
    pub product_category: ProductCategory,
}

pub struct CreateProductCategoryUsecase {
    pool: PgPool,
    product_category_repository: ProductCategoryRepoImpl,
}

impl CreateProductCategoryUsecase {
    pub fn new(pool: PgPool) -> Self {
        let product_category_repository = ProductCategoryRepoImpl;
        Self {
            pool,
            product_category_repository,
        }
    }
}

#[async_trait]
impl Usecase<CreateProductCategoryInput, CreateProductCategoryOutput, UsecaseError>
    for CreateProductCategoryUsecase
{
    async fn execute(
        &self,
        input: CreateProductCategoryInput,
    ) -> Result<CreateProductCategoryOutput, UsecaseError> {
        let category = ProductCategory {
            id: Uuid::now_v7(),
            name: input.name,
            parent_product_category_id: input.parent_product_category_id,
        };

        let created = self
            .product_category_repository
            .create(&self.pool, category)
            .await
            .map_err(UsecaseError::from)?;

        Ok(CreateProductCategoryOutput {
            product_category: created,
        })
    }
}
