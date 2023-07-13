use perroute_commons::types::actor::Actor;
use perroute_storage::models::api_key::ApiKey;

use crate::query_bus::{
    bus::{QueryBusContext, QueryHandler},
    error::QueryBusError,
    queries::FindApiKeyQuery,
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
        todo!()
    }
}
