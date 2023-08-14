use crate::{
    query,
    query_bus::{
        bus::{QueryBusContext, QueryHandler},
        error::QueryBusError,
        queries::QueryType,
    },
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
    query::FetchableModel,
};

query!(
    FindBusinessUnitQuery,
    QueryType::FindBusinessUnit,
    business_unit_id: Option<Id>
);
pub struct FindBusinessUnitQueryHandler;

#[async_trait]
impl QueryHandler for FindBusinessUnitQueryHandler {
    type Query = FindBusinessUnitQuery;
    type Output = Option<BusinessUnit>;

    #[tracing::instrument(name = "find_business_unit_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        actor: &Actor,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        BusinessUnit::find(
            ctx.pool(),
            BusinessUnitQueryBuilder::default()
                .id(*query.business_unit_id())
                .build()
                .unwrap(),
        )
        .await
        .map_err(Into::into)
    }
}
