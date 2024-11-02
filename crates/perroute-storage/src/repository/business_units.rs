use super::{PgRepository, RepositoryResult};
use crate::{execute, fetch_all, fetch_one, fetch_optional, models::business_unit::BusinessUnit};
use perroute_commons::types::{code::Code, id::Id};
use sqlx::{postgres::PgQueryResult, query, query_as};
use std::future::Future;

#[derive(Debug)]
pub enum BusinessUnitQuery {
    ByCode(Code),
    ById(Id),
    All,
}

pub trait BusinessUnitRepository {
    fn find_business_unit(
        &self,
        id: &Id,
    ) -> impl Future<Output = RepositoryResult<Option<BusinessUnit>>>;

    fn delete_business_unit(&self, id: &Id) -> impl Future<Output = RepositoryResult<bool>>;

    fn save_business_unit(
        &self,
        business_unit: BusinessUnit,
    ) -> impl Future<Output = RepositoryResult<BusinessUnit>>;

    fn update_business_unit(
        &self,
        business_unit: BusinessUnit,
    ) -> impl Future<Output = RepositoryResult<BusinessUnit>>;

    fn query_business_units(
        &self,
        query: &BusinessUnitQuery,
    ) -> impl Future<Output = RepositoryResult<Vec<BusinessUnit>>>;

    fn exists_business_unit(
        &self,
        query: &BusinessUnitQuery,
    ) -> impl Future<Output = RepositoryResult<bool>>;
}

impl BusinessUnitRepository for PgRepository {
    async fn find_business_unit(&self, id: &Id) -> RepositoryResult<Option<BusinessUnit>> {
        Ok(fetch_optional!(
            &self.source,
            query_as("select * from business_units where id = $1").bind(id)
        )?)
    }

    async fn update_business_unit(
        &self,
        business_unit: BusinessUnit,
    ) -> RepositoryResult<BusinessUnit> {
        let query = query_as("update business_units set name = $1, vars = $2, updated_at = $3 where id = $4 returning *")
            .bind(&business_unit.name)
            .bind(&business_unit.vars)
            .bind(&business_unit.updated_at)
            .bind(&business_unit.id);
        Ok(fetch_one!(&self.source, query)?)
    }

    async fn delete_business_unit(&self, id: &Id) -> RepositoryResult<bool> {
        let result: PgQueryResult = execute!(
            &self.source,
            query("delete from business_units where id = $1").bind(id)
        )?;
        Ok(result.rows_affected() > 0)
    }

    async fn save_business_unit(
        &self,
        business_unit: BusinessUnit,
    ) -> RepositoryResult<BusinessUnit> {
        let query = query_as("insert into business_units (id, code, name, vars, created_at, updated_at) values ($1, $2, $3, $4, $5, $6) returning *")
                .bind(&business_unit.id)
                .bind(&business_unit.code)
                .bind(&business_unit.name)
                .bind(&business_unit.vars)
                .bind(&business_unit.created_at)
                .bind(&business_unit.updated_at);

        Ok(fetch_one!(&self.source, query)?)
    }

    async fn exists_business_unit(&self, query: &BusinessUnitQuery) -> RepositoryResult<bool> {
        match query {
            BusinessUnitQuery::ById(id) => {
                let result: Vec<BusinessUnit> = fetch_all!(
                    &self.source,
                    query_as("select * from business_units where id = $1").bind(id)
                )?;
                Ok(!result.is_empty())
            }
            BusinessUnitQuery::ByCode(code) => {
                let result: Vec<BusinessUnit> = fetch_all!(
                    &self.source,
                    query_as("select * from business_units where code = $1").bind(code)
                )?;
                Ok(!result.is_empty())
            }
            BusinessUnitQuery::All => {
                let result: Vec<BusinessUnit> =
                    fetch_all!(&self.source, query_as("select * from business_units"))?;
                Ok(!result.is_empty())
            }
        }
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
            BusinessUnitQuery::ByCode(code) => Ok(fetch_all!(
                &self.source,
                query_as("select * from business_units where code = $1").bind(code)
            )?),
        }
    }
}
