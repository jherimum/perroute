use sqlx::query_as;
use std::future::Future;

use super::{PgRepository, RepositoryResult};
use crate::{fetch_one, models::business_unit::BusinessUnit};

pub trait BusinessUnitRepository {
    fn save_business_unit(
        &self,
        business_unit: BusinessUnit,
    ) -> impl Future<Output = RepositoryResult<BusinessUnit>>;
}

impl BusinessUnitRepository for PgRepository {
    async fn save_business_unit(
        &self,
        business_unit: BusinessUnit,
    ) -> RepositoryResult<BusinessUnit> {
        Ok(fetch_one!(
            &self.source,
            query_as("insert into ")
                .bind(&business_unit.id)
                .bind(&business_unit.code)
                .bind(&business_unit.name)
                .bind(&business_unit.created_at)
                .bind(&business_unit.updated_at)
        )?)
    }
}
