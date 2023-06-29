use async_trait::async_trait;
use perroute_storage::models::channel::Channel;

use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::FindChannelByCodeQuery,
};

pub struct FindChannelByCodeHandler;

#[async_trait]
impl QueryHandler for FindChannelByCodeHandler {
    type Query = FindChannelByCodeQuery;
    type Output = Option<Channel>;

    #[tracing::instrument(name = "find_channel_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Channel::find_by_code(ctx.pool(), query.channel_code())
            .await
            .map_err(Into::into)
    }
}
