use super::models::{
    CreateRouteRequest, RouteCollectionPath, RouteModel, RoutePath, UpdateRouteRequest,
};
use crate::rest::{
    models::{ResourceModel, ResourceModelCollection},
    modules::ApiResult,
    service::RestService,
};
use perroute_command_bus::CommandBus;
use perroute_commons::types::actor::Actor;
use perroute_query_bus::QueryBus;
use std::future::Future;

pub trait RouteRestService {
    fn create(
        &self,
        actor: &Actor,
        path: &RouteCollectionPath,
        _req: &CreateRouteRequest,
    ) -> impl Future<Output = ApiResult<ResourceModel<RouteModel>>>;

    fn update(
        &self,
        actor: &Actor,
        path: &RoutePath,
        _req: &UpdateRouteRequest,
    ) -> impl Future<Output = ApiResult<ResourceModel<RouteModel>>>;

    fn get(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> impl Future<Output = ApiResult<Option<ResourceModel<RouteModel>>>>;

    fn query(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> impl Future<Output = ApiResult<ResourceModelCollection<RouteModel>>>;

    fn delete(&self, actor: &Actor, path: &RoutePath) -> impl Future<Output = ApiResult<bool>>;
}

impl<CB: CommandBus, QB: QueryBus> RouteRestService for RestService<CB, QB> {
    async fn create(
        &self,
        actor: &Actor,
        path: &RouteCollectionPath,
        _req: &CreateRouteRequest,
    ) -> ApiResult<ResourceModel<RouteModel>> {
        todo!()
    }

    async fn update(
        &self,
        actor: &Actor,
        path: &RoutePath,
        _req: &UpdateRouteRequest,
    ) -> ApiResult<ResourceModel<RouteModel>> {
        todo!()
    }

    async fn get(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> ApiResult<Option<ResourceModel<RouteModel>>> {
        todo!()
    }

    async fn delete(&self, actor: &Actor, path: &RoutePath) -> ApiResult<bool> {
        todo!()
    }

    async fn query(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> ApiResult<ResourceModelCollection<RouteModel>> {
        todo!()
    }
}
