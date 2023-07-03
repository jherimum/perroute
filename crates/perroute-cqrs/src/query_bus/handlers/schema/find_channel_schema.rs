use crate::query_bus::queries::FindChannelSchemaQuery;
use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
};
use async_trait::async_trait;
use perroute_storage::models::schema::Schema;

pub struct FindChannelSchemaQueryHandler;

#[async_trait]
impl QueryHandler for FindChannelSchemaQueryHandler {
    type Query = FindChannelSchemaQuery;
    type Output = Option<Schema>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        todo!()
    }
}
