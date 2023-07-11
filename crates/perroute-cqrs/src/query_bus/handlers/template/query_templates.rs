use async_trait::async_trait;
use perroute_storage::{
    models::template::{Template, TemplatesQueryBuilder},
    query::FetchableModel,
};

use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::QueryTemplatesQuery,
};

pub struct QueryTemplatesQueryHandler;

#[async_trait]
impl QueryHandler for QueryTemplatesQueryHandler {
    type Query = QueryTemplatesQuery;
    type Output = Vec<Template>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Template::query(
            ctx.pool(),
            TemplatesQueryBuilder::default()
                .schema_id(*query.schema_id())
                .build()
                .unwrap(),
        )
        .await
        .map_err(Into::into)
    }
}
