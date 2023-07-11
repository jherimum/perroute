use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::QueryMessageTypesQuery,
};
use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::message_type::{MessageType, MessageTypeQueryBuilder},
    query::FetchableModel,
};

pub struct QueryMessageTypesHandler;

#[async_trait]
impl QueryHandler for QueryMessageTypesHandler {
    type Query = QueryMessageTypesQuery;
    type Output = Vec<MessageType>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        _: &Actor,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        MessageType::query(
            ctx.pool(),
            MessageTypeQueryBuilder::default()
                .channel_id(*query.channel_id())
                .build()
                .unwrap(),
        )
        .await
        .map_err(Into::into)
    }
}
