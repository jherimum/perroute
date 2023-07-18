use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::QuerySchemasQuery,
};
use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::schema::{Schema, SchemasQueryBuilder},
    query::FetchableModel,
};

#[derive(Debug)]
pub struct QuerySchemasQueryHandler;

#[async_trait]
impl QueryHandler for QuerySchemasQueryHandler {
    type Output = Vec<Schema>;
    type Query = QuerySchemasQuery;

    #[tracing::instrument(name = "query_schemas_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        _: &Actor,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Schema::query(
            ctx.pool(),
            SchemasQueryBuilder::default()
                .message_type_id(Some(*query.message_type_id()))
                .build()
                .unwrap(),
        )
        .await
        .map_err(Into::into)
    }
}
