use crate::{
    impl_query,
    query_bus::{
        bus::{QueryBusContext, QueryHandler},
        queries::QueryType,
        Result,
    },
};
use async_trait::async_trait;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_storage::{
    models::message_type::{MessageType, MessageTypeQueryBuilder},
    query::FetchableModel,
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct QueryMessageTypesQuery {}

impl_query!(QueryMessageTypesQuery, QueryType::QueryMessageTypes);

pub struct QueryMessageTypesHandler;

#[async_trait]
impl QueryHandler for QueryMessageTypesHandler {
    type Query = QueryMessageTypesQuery;
    type Output = Vec<MessageType>;

    #[tracing::instrument(name = "query_message_types_handler", skip(self, ctx))]
    async fn handle(&self, ctx: &QueryBusContext, query: &Self::Query) -> Result<Self::Output> {
        MessageType::query(
            ctx.pool(),
            MessageTypeQueryBuilder::default().build().unwrap(),
        )
        .await
        .map_err(Into::into)
    }
}
