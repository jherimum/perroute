use async_trait::async_trait;
use perroute_storage::models::template::Template;

use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::QueryTemplatesQuery,
};

pub struct QueryTemplatesQueryHandler;

#[async_trait]
impl QueryHandler for QueryTemplatesQueryHandler {
    type Query = QueryTemplatesQuery;
    type Output = Option<Template>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        todo!()
    }
}
