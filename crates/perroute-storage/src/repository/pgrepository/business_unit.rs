use crate::{
    active_record::business_unit::{BusinessUnitQuery, CreateBusinessUnit},
    models::business_unit::BusinessUnit,
    repository::{business_unit::BusinessUnitRepository, RepositoryResult},
};
use super::PgRepository;

#[async_trait::async_trait]
impl BusinessUnitRepository for PgRepository {
    async fn create_bu(
        &self,
        create: CreateBusinessUnit,
    ) -> RepositoryResult<BusinessUnit> {
        todo!()
    }

    async fn update_bu(
        &self,
        business_unit: BusinessUnit,
    ) -> RepositoryResult<BusinessUnit> {
        todo!()
    }

    async fn get_bu<'q>(
        &self,
        query: BusinessUnitQuery<'q>,
    ) -> RepositoryResult<BusinessUnit> {
        todo!()
    }
}
