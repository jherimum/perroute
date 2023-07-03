use async_trait::async_trait;
use perroute_storage::models::schema::Schema;

use crate::query_bus::queries::FindSchemaByIdQuery;
use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
};

pub struct FindSchemaByIdQueryHandler;

#[async_trait]
impl QueryHandler for FindSchemaByIdQueryHandler {
    type Query = FindSchemaByIdQuery;
    type Output = Option<Schema>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        todo!()
    }
}
