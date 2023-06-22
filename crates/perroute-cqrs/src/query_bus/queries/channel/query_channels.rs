use async_trait::async_trait;
use derive_builder::Builder;
use perroute_storage::models::channel::Channel;
use serde::Serialize;

use crate::query_bus::{
    bus::{Query, QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::QueryType,
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Builder)]
pub struct QueryChannelsQuery {}

pub struct QueryChannelsQueryHandler;

impl Query for QueryChannelsQuery {
    fn ty(&self) -> QueryType {
        QueryType::QueryChannels
    }
}

#[async_trait]
impl QueryHandler for QueryChannelsQueryHandler {
    type Query = QueryChannelsQuery;
    type Output = Vec<Channel>;

    #[tracing::instrument(name = "querys_channels_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        _: Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Channel::query(ctx.pool())
            .await
            .map_err(QueryBusError::from)
    }
}
