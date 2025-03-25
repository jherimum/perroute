use crate::{
    models::route::Route,
    repository::{route::RouteRepository, RepositoryResult},
};
use super::PgRepository;
use perroute_commons::types::{id::Id, dispatch_type::DispatchType};

#[async_trait::async_trait]
impl RouteRepository for PgRepository {
    async fn routes_to_dispatch(
        &self,
        business_unit_id: &Id,
        message_type_id: &Id,
        dispatch_type: &DispatchType,
    ) -> RepositoryResult<Vec<Route>> {
        todo!()
    }
}
