use crate::query_bus::queries::FindChannelMessageTypeSchemaQuery;
use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
};
use async_trait::async_trait;
use perroute_storage::models::schema::Schema;

pub struct FindChannelSchemaQueryHandler;

#[async_trait]
impl QueryHandler for FindChannelSchemaQueryHandler {
    type Query = FindChannelMessageTypeSchemaQuery;
    type Output = Option<Schema>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        todo!()
    }
}
