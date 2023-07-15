use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::FindApiKeyQuery,
};
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::api_key::{ApiKey, ApiKeyQueryBuilder},
    query::FetchableModel,
};

pub struct FindApiKeyQueryHandler;

#[async_trait::async_trait]
impl QueryHandler for FindApiKeyQueryHandler {
    type Query = FindApiKeyQuery;
    type Output = Option<ApiKey>;

    async fn handle(
        &self,
        ctx: &QueryBusContext,
        actor: &Actor,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        let query = ApiKeyQueryBuilder::default()
            .id(*query.api_key_id())
            .hash(query.hash().clone())
            .build()
            .unwrap();
        ApiKey::find(ctx.pool(), query)
            .await
            .map_err(QueryBusError::from)
    }
}
