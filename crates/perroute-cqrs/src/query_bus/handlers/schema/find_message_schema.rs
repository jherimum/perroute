use async_trait::async_trait;
use perroute_storage::models::schema::Schema;

use crate::query_bus::queries::FindMessageTypeSchemaQuery;
use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
};

pub struct FindMessageTypeSchemaQueryHandler;

#[async_trait]
impl QueryHandler for FindMessageTypeSchemaQueryHandler {
    type Query = FindMessageTypeSchemaQuery;
    type Output = Option<Schema>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        // Schema::find_message_type_id_and_id(ctx.pool(), query.message_type_id(), query.schema_id())
        //     .await
        //     .map_err(Into::into)
        todo!()
    }
}
