use super::{PgRepository, RepositoryResult};
use crate::{execute, fetch_optional, models::route::Route};
use perroute_commons::types::id::Id;
use std::future::Future;

pub enum RouteQuery<'q> {
    ById(&'q Id),
}

pub trait RouteRepository {
    fn get(&self, query: &RouteQuery<'_>) -> impl Future<Output = RepositoryResult<Option<Route>>>;

    fn save(&self, route: Route) -> impl Future<Output = RepositoryResult<Route>>;

    fn update(&self, route: Route) -> impl Future<Output = RepositoryResult<Route>>;

    fn delete(&self, query: &RouteQuery<'_>) -> impl Future<Output = RepositoryResult<u64>>;
}

impl RouteRepository for PgRepository {
    async fn save(&self, route: Route) -> RepositoryResult<Route> {
        todo!()
    }

    async fn update(&self, route: Route) -> RepositoryResult<Route> {
        todo!()
    }

    async fn delete(&self, query: &RouteQuery<'_>) -> RepositoryResult<u64> {
        match query {
            RouteQuery::ById(id) => {
                let query = sqlx::query("DELETE FROM routes WHERE id = $1").bind(id);
                Ok(execute!(&self.source, query)?.rows_affected())
            }
        }
    }

    async fn get(&self, query: &RouteQuery<'_>) -> RepositoryResult<Option<Route>> {
        match query {
            RouteQuery::ById(id) => {
                let query = sqlx::query_as("SELECT * FROM routes WHERE id = $1").bind(id);
                Ok(fetch_optional!(&self.source, query)?)
            }
        }
    }
}
