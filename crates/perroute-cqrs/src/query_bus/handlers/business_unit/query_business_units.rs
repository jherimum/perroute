use crate::{
    query,
    query_bus::{
        bus::{QueryBusContext, QueryHandler},
        error::QueryBusError,
        queries::QueryType,
        Result,
    },
};
use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
    query::FetchableModel,
};

query!(QueryBusinessUnitsQuery, QueryType::QueryBusinessUnits);

pub struct QueryBusinessUnitsQueryHandler;

#[async_trait]
impl QueryHandler for QueryBusinessUnitsQueryHandler {
    type Query = QueryBusinessUnitsQuery;
    type Output = Vec<BusinessUnit>;

    #[tracing::instrument(name = "query_business_units_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        actor: &Actor,
        _: &Self::Query,
    ) -> Result<Self::Output> {
        BusinessUnit::query(
            ctx.pool(),
            BusinessUnitQueryBuilder::default().build().unwrap(),
        )
        .await
        .map_err(QueryBusError::from)
    }
}
