use crate::{
    active_record::business_unit::{BusinessUnitQuery, CreateBusinessUnit},
    models::business_unit::BusinessUnit,
};
use super::RepositoryResult;

#[async_trait::async_trait]
pub trait BusinessUnitRepository {
    async fn create_bu(
        &self,
        create: CreateBusinessUnit,
    ) -> RepositoryResult<BusinessUnit>;

    async fn update_bu(
        &self,
        business_unit: BusinessUnit,
    ) -> RepositoryResult<BusinessUnit>;

    async fn get_bu<'q>(
        &self,
        query: BusinessUnitQuery<'q>,
    ) -> RepositoryResult<BusinessUnit>;
}
