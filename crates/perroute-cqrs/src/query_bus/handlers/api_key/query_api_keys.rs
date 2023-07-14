use perroute_commons::types::actor::Actor;
use perroute_storage::models::api_key::ApiKey;

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
        todo!()
    }
}
