use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::QueryMessageTypesQuery,
};
use async_trait::async_trait;
use perroute_storage::models::message_type::MessageType;

pub struct QueryMessageTypesHandler;

#[async_trait]
impl QueryHandler for QueryMessageTypesHandler {
    type Query = QueryMessageTypesQuery;
    type Output = Vec<MessageType>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        _: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        MessageType::query(ctx.pool()).await.map_err(Into::into)
    }
}
