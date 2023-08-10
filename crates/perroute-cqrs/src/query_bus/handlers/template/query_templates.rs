use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::template::{Template, TemplatesQueryBuilder},
    query::FetchableModel,
};

use crate::{
    query,
    query_bus::{
        bus::{QueryBusContext, QueryHandler},
        error::QueryBusError,
        queries::QueryType,
    },
};

query!(QueryTemplatesQuery, QueryType::QueryTemplates,);

pub struct QueryTemplatesQueryHandler;

#[async_trait]
impl QueryHandler for QueryTemplatesQueryHandler {
    type Query = QueryTemplatesQuery;
    type Output = Vec<Template>;

    #[tracing::instrument(name = "query_templates_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        _: &Actor,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Template::query(
            ctx.pool(),
            TemplatesQueryBuilder::default().build().unwrap(),
        )
        .await
        .map_err(Into::into)
    }
}
