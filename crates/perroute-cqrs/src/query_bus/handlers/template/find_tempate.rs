use crate::{
    query,
    query_bus::{
        bus::{QueryBusContext, QueryHandler},
        error::QueryBusError,
        queries::QueryType,
    },
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::template::{Template, TemplatesQueryBuilder},
    query::FetchableModel,
};

query!(
    FindTemplateQuery,
    QueryType::FindTemplate,
    template_id: Id,
    message_type_id: Option<Id>,
    schema_id: Option<Id>,
    bu_id: Option<Id>
);

pub struct FindTemplateQueryHandler;

#[async_trait]
impl QueryHandler for FindTemplateQueryHandler {
    type Query = FindTemplateQuery;
    type Output = Option<Template>;

    #[tracing::instrument(name = "find_template_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        _: &Actor,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        Template::find(
            ctx.pool(),
            TemplatesQueryBuilder::default()
                .id(Some(*query.template_id()))
                .bu_id(*query.bu_id())
                .build()
                .unwrap(),
        )
        .await
        .map_err(Into::into)
    }
}
