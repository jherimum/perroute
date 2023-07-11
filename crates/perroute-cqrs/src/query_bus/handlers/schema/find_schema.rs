use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::schema::{Schema, SchemasQueryBuilder},
    query::FetchableModel,
};

use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::FindSchemaQuery,
};

pub struct FindSchemaQueryHandler;

#[async_trait]
impl QueryHandler for FindSchemaQueryHandler {
    type Query = FindSchemaQuery;
    type Output = Option<Schema>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        _: &Actor,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        let query = SchemasQueryBuilder::default()
            .id(Some(*query.schema_id()))
            .channel_id(*query.channel_id())
            .message_type_id(*query.message_type_id())
            .build()
            .unwrap();

        Schema::find(ctx.pool(), query).await.map_err(Into::into)
    }
}
