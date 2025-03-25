use crate::{Query, QueryBusContext, QueryBusResult, QueryHandler};
use perroute_storage::{
    models::business_unit::BusinessUnit,
    repository::business_units::{BusinessUnitQuery, BusinessUnitRepository},
};

impl Query for BusinessUnitQuery {}

pub struct QueryBusinessUnitsHandler;

impl QueryHandler for QueryBusinessUnitsHandler {
    type Query = BusinessUnitQuery;

    type Output = Vec<BusinessUnit>;

    async fn handle<R: perroute_storage::repository::Repository + Clone>(
        &self,
        query: &Self::Query,
        ctx: QueryBusContext<'_, R>,
    ) -> QueryBusResult<Self::Output> {
        Ok(
            BusinessUnitRepository::query_business_units(ctx.repository, query)
                .await
                .unwrap(),
        )
    }
}
