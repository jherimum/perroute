use perroute_commons::types::id::Id;
use sqlx::query_as;
use std::future::Future;

use super::{PgRepository, RepositoryResult};
use crate::{fetch_all, fetch_one, models::business_unit::BusinessUnit};

pub enum BusinessUnitQuery {
    ById(Id),
    All,
}

pub trait BusinessUnitRepository {
    fn save_business_unit(
        &self,
        business_unit: BusinessUnit,
    ) -> impl Future<Output = RepositoryResult<BusinessUnit>>;

    fn query_business_units(
        &self,
        query: &BusinessUnitQuery,
    ) -> impl Future<Output = RepositoryResult<Vec<BusinessUnit>>>;
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

    async fn query_business_units(
        &self,
        query: &BusinessUnitQuery,
    ) -> RepositoryResult<Vec<BusinessUnit>> {
        match query {
            BusinessUnitQuery::ById(id) => Ok(fetch_all!(
                &self.source,
                query_as("select * from business_units where id = $1").bind(id)
            )?),
            BusinessUnitQuery::All => Ok(fetch_all!(
                &self.source,
                query_as("select * from business_units")
            )?),
        }
    }
}
