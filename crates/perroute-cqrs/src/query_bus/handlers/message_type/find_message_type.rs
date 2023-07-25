use async_trait::async_trait;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::message_type::{MessageType, MessageTypeQueryBuilder},
    query::FetchableModel,
};
use serde::Serialize;

use crate::{
    impl_query,
    query_bus::{
        bus::{QueryBusContext, QueryHandler},
        error::QueryBusError,
        queries::QueryType,
    },
};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct FindMessageTypeQuery {
    message_type_id: Id,
    #[builder(default)]
    channel_id: Option<Id>,
}

impl_query!(FindMessageTypeQuery, QueryType::FindMessageTypeQuery);

pub struct FindMessageTypeQueryHandler;

#[async_trait]
impl QueryHandler for FindMessageTypeQueryHandler {
    type Query = FindMessageTypeQuery;
    type Output = Option<MessageType>;

    #[tracing::instrument(name = "find_message_type_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        _: &Actor,
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
