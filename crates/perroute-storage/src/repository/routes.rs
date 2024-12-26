use super::{PgRepository, RepositoryResult};
use crate::{execute, fetch_optional, models::route::Route};
use perroute_commons::types::{dispatch_type::DispatchType, id::Id};

pub enum RouteQuery<'q> {
    ById(&'q Id),
    ActiveByBusinessUnitAndDispatchType(&'q ActiveByBusinessUnitAndDispatchTypeQuery<'q>),
}

pub struct ActiveByBusinessUnitAndDispatchTypeQuery<'q> {
    pub business_unit_id: &'q Id,
    pub message_type_id: &'q Id,
    pub dispatch_type: &'q DispatchType,
}

#[async_trait::async_trait]
pub trait RouteRepository {
    async fn get(&self, query: &RouteQuery<'_>) -> RepositoryResult<Option<Route>>;

    async fn save(&self, route: Route) -> RepositoryResult<Route>;

    async fn update(&self, route: Route) -> RepositoryResult<Route>;

    async fn delete(&self, query: &RouteQuery<'_>) -> RepositoryResult<u64>;

    async fn query(&self, query: &RouteQuery<'_>) -> RepositoryResult<Vec<Route>>;
}

#[async_trait::async_trait]
impl RouteRepository for PgRepository {
    async fn query(&self, query: &RouteQuery<'_>) -> RepositoryResult<Vec<Route>> {
        todo!()
    }

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
            _ => todo!(),
        }
    }

    async fn get(&self, query: &RouteQuery<'_>) -> RepositoryResult<Option<Route>> {
        match query {
            RouteQuery::ById(id) => {
                let query = sqlx::query_as("SELECT * FROM routes WHERE id = $1").bind(id);
                Ok(fetch_optional!(&self.source, query)?)
            }
            _ => todo!(),
        }
    }
}
