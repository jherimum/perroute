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
use perroute_commons::types::id::Id;
use perroute_storage::{
    models::business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
    query::FetchableModel,
};

query!(
    FindBusinessUnitQuery,
    QueryType::FindBusinessUnit,
    id: Id
);
pub struct FindBusinessUnitQueryHandler;

#[async_trait]
impl QueryHandler for FindBusinessUnitQueryHandler {
    type Query = FindBusinessUnitQuery;
    type Output = BusinessUnit;

    #[tracing::instrument(name = "find_business_unit_handler", skip(self, ctx))]
    async fn handle(&self, ctx: &QueryBusContext, query: &Self::Query) -> Result<Self::Output> {
        BusinessUnit::find(
            ctx.pool(),
            BusinessUnitQueryBuilder::default()
                .id(Some(query.id))
                .build()
                .unwrap(),
        )
        .await?
        .ok_or(QueryBusError::EntityNotFound("BusinessUnit".to_owned()))
    }
}
