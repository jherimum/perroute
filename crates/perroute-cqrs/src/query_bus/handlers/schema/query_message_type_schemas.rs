use async_trait::async_trait;
use perroute_storage::models::schema::Schema;

use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::QueryMessageTypeSchemasQuery,
};

#[derive(Debug)]
pub struct QueryMessageTypeSchemasQueryHandler;

#[async_trait]
impl QueryHandler for QueryMessageTypeSchemasQueryHandler {
    type Output = Vec<Schema>;
    type Query = QueryMessageTypeSchemasQuery;

    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Schema::query(ctx.pool(), query.message_type_id())
            .await
            .map_err(Into::into)
    }
}
