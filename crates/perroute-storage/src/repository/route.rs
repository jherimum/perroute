use perroute_commons::types::{dispatch_type::DispatchType, id::Id};
use crate::models::route::Route;
use super::RepositoryResult;

#[async_trait::async_trait]
pub trait RouteRepository {
    async fn routes_to_dispatch(
        &self,
        business_unit_id: &Id,
        message_type_id: &Id,
        dispatch_type: &DispatchType,
    ) -> RepositoryResult<Vec<Route>>;
}
