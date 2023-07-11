use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::FindTemplateQuery,
};
use async_trait::async_trait;
use perroute_storage::{
    models::template::{Template, TemplatesQueryBuilder},
    query::FetchableModel,
};

pub struct FindTemplateQueryHandler;

#[async_trait]
impl QueryHandler for FindTemplateQueryHandler {
    type Query = FindTemplateQuery;
    type Output = Option<Template>;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Template::find(
            ctx.pool(),
            TemplatesQueryBuilder::default()
                .id(Some(*query.template_id()))
                .schema_id(*query.schema_id())
                .message_type_id(*query.message_type_id())
                .channel_id(*query.channel_id())
                .build()
                .unwrap(),
        )
        .await
        .map_err(Into::into)
    }
}
