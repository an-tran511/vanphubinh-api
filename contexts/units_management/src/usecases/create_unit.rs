use async_trait::async_trait;
use derive_more::From;
use shared::{error::UsecaseError, usecase::Usecase};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::unit::{Unit, UnitRepository},
    infra::repos::unit_repo::UnitRepoImpl,
};

#[derive(From)]
pub struct CreateUnitInput {
    pub name: String,
}

pub struct CreateUnitOutput {
    pub unit: Unit,
}

pub struct CreateUnitUsecase {
    pool: PgPool,
    unit_repository: UnitRepoImpl,
}

impl CreateUnitUsecase {
    pub fn new(pool: PgPool) -> Self {
        let unit_repository = UnitRepoImpl;

        Self {
            pool,
            unit_repository,
        }
    }
}

#[async_trait]
impl Usecase<CreateUnitInput, CreateUnitOutput, UsecaseError> for CreateUnitUsecase {
    async fn execute(&self, input: CreateUnitInput) -> Result<CreateUnitOutput, UsecaseError> {
        let unit = Unit {
            id: Uuid::now_v7(),
            name: input.name,
        };

        let created_unit = self
            .unit_repository
            .create(&self.pool, unit)
            .await
            .map_err(UsecaseError::from)?;

        Ok(CreateUnitOutput { unit: created_unit })
    }
}
