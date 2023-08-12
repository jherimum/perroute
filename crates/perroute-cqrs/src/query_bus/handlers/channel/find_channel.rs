use crate::{
    query,
    query_bus::{
        bus::{QueryBusContext, QueryHandler},
        error::QueryBusError,
        queries::QueryType,
    },
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::channel::{Channel, ChannelsQueryBuilder},
    query::FetchableModel,
};

query!(
    FindChannelQuery,
    QueryType::FindChannel,
    channel_id: Option<Id>
);
pub struct FindChannelQueryHandler;

#[async_trait]
impl QueryHandler for FindChannelQueryHandler {
    type Query = FindChannelQuery;
    type Output = Option<Channel>;

    #[tracing::instrument(name = "find_channel_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        actor: &Actor,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Channel::find(
            ctx.pool(),
            ChannelsQueryBuilder::default()
                .id(*query.channel_id())
                .build()
                .unwrap(),
        )
        .await
        .map_err(Into::into)
    }
}
