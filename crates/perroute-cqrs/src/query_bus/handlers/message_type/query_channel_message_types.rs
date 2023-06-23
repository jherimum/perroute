use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::QueryChannelMessageTypesQuery,
};
use async_trait::async_trait;
use perroute_storage::models::message_type::MessageType;

pub struct QueryChannelMessageTypesHandler;

#[async_trait]
impl QueryHandler for QueryChannelMessageTypesHandler {
    type Query = QueryChannelMessageTypesQuery;
    type Output = Vec<MessageType>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        MessageType::query_by_channel_id(ctx.pool(), query.channel_id())
            .await
            .map_err(Into::into)
    }
}
