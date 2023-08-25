use crate::{
    impl_query,
    query_bus::{
        bus::{QueryBusContext, QueryHandler},
        error::QueryBusError,
        queries::QueryType,
    },
};
use async_trait::async_trait;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, code::Code, id::Id, version::Version};
use perroute_storage::{
    models::schema::{Schema, SchemasQueryBuilder},
    query::FetchableModel,
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct FindSchemaQuery {
    #[builder(default)]
    business_unit_id: Option<Id>,
    #[builder(default)]
    message_type_id: Option<Id>,
    #[builder(default)]
    version: Option<Version>,
    #[builder(default)]
    schema_id: Option<Id>,
    #[builder(default)]
    bu_code: Option<Code>,
}

impl_query!(FindSchemaQuery, QueryType::FindSchema);

pub struct FindSchemaQueryHandler;

#[async_trait]
impl QueryHandler for FindSchemaQueryHandler {
    type Query = FindSchemaQuery;
    type Output = Schema;

    #[tracing::instrument(name = "find_schema_handler", skip(self, ctx))]
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        _: &Actor,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        let query = SchemasQueryBuilder::default()
            .id(*query.schema_id())
            .message_type_id(*query.message_type_id())
            .version(*query.version())
            .build()
            .unwrap();

        Schema::find(ctx.pool(), query)
            .await?
            .ok_or(QueryBusError::EntityNotFound("".to_owned()))
    }
}
