use crate::{
    query,
    query_bus::{
        bus::{QueryBusContext, QueryHandler},
        queries::QueryType,
        Result,
    },
};
use async_trait::async_trait;
use perroute_storage::{
    models::connection::{Connection, ConnectionQueryBuilder},
    query::FetchableModel,
};

query!(QueryConnectionsQuery, QueryType::QueryConnections);

pub struct QueryConnectionsQueryHandler;

#[async_trait]
impl QueryHandler for QueryConnectionsQueryHandler {
    type Query = QueryConnectionsQuery;
    type Output = Vec<Connection>;

    #[tracing::instrument(name = "query_Connections_handler", skip(self, ctx))]
    async fn handle(&self, ctx: &QueryBusContext, _: &Self::Query) -> Result<Self::Output> {
        Ok(Connection::query(
            ctx.pool(),
            ConnectionQueryBuilder::default().build().unwrap(),
        )
        .await?)
    }
}
