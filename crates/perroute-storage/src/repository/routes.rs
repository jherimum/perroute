use std::future::Future;

use crate::models::route::Route;

use super::{PgRepository, RepositoryResult};

pub trait RouteRepository {
    fn save(&self, route: Route) -> impl Future<Output = RepositoryResult<Route>>;
}

impl RouteRepository for PgRepository {
    async fn save(&self, route: Route) -> RepositoryResult<Route> {
        todo!()
    }
}
