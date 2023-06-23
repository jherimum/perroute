use async_trait::async_trait;
use perroute_storage::models::channel::Channel;

use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::FindChannelQuery,
};

pub struct FindChannelQueryHandler;

#[async_trait]
impl QueryHandler for FindChannelQueryHandler {
    type Query = FindChannelQuery;
    type Output = Option<Channel>;

    #[tracing::instrument(name = "find_channel_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Channel::find_by_id(ctx.pool(), query.channel_id())
            .await
            .map_err(Into::into)
    }
}
