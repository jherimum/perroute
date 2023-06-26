use async_trait::async_trait;
use perroute_storage::models::schema::Schema;

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
        query: Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Schema::find_by_id(ctx.pool(), query.message_type_id())
            .await
            .map_err(Into::into)
    }
}
