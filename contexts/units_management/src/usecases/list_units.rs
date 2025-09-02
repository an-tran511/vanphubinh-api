use async_trait::async_trait;
use derive_more::From;
use shared::{
    common_types::{PaginationInput, PaginationMeta},
    error::UsecaseError,
    usecase::Usecase,
};
use sqlx::PgPool;

use crate::{
    domain::unit::{Unit, UnitRepository},
    infra::repos::unit_repo::UnitRepoImpl,
};

#[derive(From)]
pub struct ListUnitsInput {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

pub struct ListUnitsOutput {
    pub units: Vec<Unit>,
    pub meta: PaginationMeta,
}

pub struct ListUnitsUsecase {
    pool: PgPool,
    unit_repository: UnitRepoImpl,
}

impl ListUnitsUsecase {
    pub fn new(pool: PgPool) -> Self {
        let unit_repository = UnitRepoImpl;

        Self {
            pool,
            unit_repository,
        }
    }
}

#[async_trait]
impl Usecase<ListUnitsInput, ListUnitsOutput, UsecaseError> for ListUnitsUsecase {
    async fn execute(&self, input: ListUnitsInput) -> Result<ListUnitsOutput, UsecaseError> {
        let (units, meta) = self
            .unit_repository
            .find_all_paginated(
                &self.pool,
                PaginationInput {
                    page: input.page,
                    per_page: input.per_page,
                },
            )
            .await?;

        Ok(ListUnitsOutput { units, meta })
    }
}
