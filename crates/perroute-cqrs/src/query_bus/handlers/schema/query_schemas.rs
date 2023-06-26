use async_trait::async_trait;
use perroute_storage::models::schema::Schema;

use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::QuerySchemasQuery,
};

#[derive(Debug)]
pub struct QuerySchemasQueryHandler;

#[async_trait]
impl QueryHandler for QuerySchemasQueryHandler {
    type Output = Vec<Schema>;
    type Query = QuerySchemasQuery;

    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Schema::query(ctx.pool(), query.message_type_id())
            .await
            .map_err(Into::into)
    }
}
