use async_trait::async_trait;
use perroute_storage::models::message_type::{MessageType, MessageTypeQueryBuilder};

use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::FindMessageTypeQuery,
};

pub struct FindMessageTypeQueryHandler;

#[async_trait]
impl QueryHandler for FindMessageTypeQueryHandler {
    type Query = FindMessageTypeQuery;
    type Output = Option<MessageType>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        MessageType::find(
            ctx.pool(),
            MessageTypeQueryBuilder::default()
                .id(Some(*query.message_type_id()))
                .channel_id(*query.channel_id())
                .build()
                .unwrap(),
        )
        .await
        .map_err(Into::into)
    }
}
