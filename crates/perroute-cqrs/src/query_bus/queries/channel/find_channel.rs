use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::id::Id;
use perroute_storage::models::channel::Channel;
use serde::Serialize;

use crate::query_bus::{
    bus::{Query, QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::QueryType,
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, new)]
pub struct FindChannelQuery {
    pub channel_id: Id,
}

pub struct FindChannelQueryHandler;

impl Query for FindChannelQuery {
    fn ty(&self) -> QueryType {
        QueryType::FindChannel
    }
}

#[async_trait]
impl QueryHandler for FindChannelQueryHandler {
    type Query = FindChannelQuery;
    type Output = Option<Channel>;

    #[tracing::instrument(name = "query_channel_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Channel::find_by_id(ctx.pool(), query.channel_id)
            .await
            .map_err(Into::into)
    }
}
