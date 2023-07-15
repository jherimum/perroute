use perroute_commons::types::actor::Actor;
use perroute_storage::models::api_key::{ApiKey, ApiKeyQuery, ApiKeyQueryBuilder};
use perroute_storage::query::{FetchableModel, ModelQueryBuilder};

use crate::query_bus::queries::QueryApiKeysQuery;
use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::FindApiKeyQuery,
};

pub struct QueryApiKeysQueryHandler;

#[async_trait::async_trait]
impl QueryHandler for QueryApiKeysQueryHandler {
    type Query = QueryApiKeysQuery;
    type Output = Vec<ApiKey>;

    async fn handle(
        &self,
        ctx: &QueryBusContext,
        actor: &Actor,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError> {
        let query = ApiKeyQueryBuilder::default()
            .channel_id(*query.channel_id())
            .hash(query.hash().clone())
            .id(*query.api_key_id())
            .build()
            .unwrap();

        ApiKey::query(ctx.pool(), query)
            .await
            .map_err(QueryBusError::from)
    }
}
