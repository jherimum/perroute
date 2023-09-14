use crate::{
    query,
    query_bus::{
        bus::{QueryBusContext, QueryHandler},
        error::QueryBusError,
        queries::QueryType,
        Result,
    },
};
use async_trait::async_trait;
use perroute_commons::types::id::Id;
use perroute_storage::{
    models::connection::{Connection, ConnectionQueryBuilder},
    query::FetchableModel,
};

query!(
    FindConnectionQuery,
    QueryType::FindConnection,
    id: Id
);
pub struct FindConnectionQueryHandler;

#[async_trait]
impl QueryHandler for FindConnectionQueryHandler {
    type Query = FindConnectionQuery;
    type Output = Connection;

    #[tracing::instrument(name = "find_Connection_handler", skip(self, ctx))]
    async fn handle(&self, ctx: &QueryBusContext, query: &Self::Query) -> Result<Self::Output> {
        Connection::find(
            ctx.pool(),
            ConnectionQueryBuilder::default()
                .id(Some(query.id))
                .build()
                .unwrap(),
        )
        .await?
        .ok_or(QueryBusError::EntityNotFound("Connection".to_owned()))
    }
}
