use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::FindSchemaQuery,
};
use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::schema::{Schema, SchemasQueryBuilder},
    query::FetchableModel,
};

pub struct FindSchemaQueryHandler;

#[async_trait]
impl QueryHandler for FindSchemaQueryHandler {
    type Query = FindSchemaQuery;
    type Output = Option<Schema>;

    #[tracing::instrument(name = "find_schema_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        _: &Actor,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        let query = SchemasQueryBuilder::default()
            .id(*query.schema_id())
            .channel_id(*query.channel_id())
            .message_type_id(*query.message_type_id())
            .version(*query.version())
            .message_type_code(query.message_type_code().clone())
            .channel_code(query.channel_code().clone())
            .build()
            .unwrap();

        Schema::find(ctx.pool(), query).await.map_err(Into::into)
    }
}
