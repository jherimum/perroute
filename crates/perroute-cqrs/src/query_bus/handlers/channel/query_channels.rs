use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::QueryChannelsQuery,
};
use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::channel::{Channel, ChannelsQueryBuilder},
    query::FetchableModel,
};

pub struct QueryChannelsQueryHandler;

#[async_trait]
impl QueryHandler for QueryChannelsQueryHandler {
    type Query = QueryChannelsQuery;
    type Output = Vec<Channel>;

    #[tracing::instrument(name = "query_channels_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        actor: &Actor,
        _: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Channel::query(ctx.pool(), ChannelsQueryBuilder::default().build().unwrap())
            .await
            .map_err(QueryBusError::from)
    }
}
