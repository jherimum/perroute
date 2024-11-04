use super::models::{
    CreateRouteRequest, RouteCollectionPath, RouteModel, RoutePath, UpdateRouteRequest,
};
use crate::rest::{
    modules::ApiResult, service::RestService, ResourceModelCollectionResult, ResourceModelResult,
};
use perroute_command_bus::{
    commands::route::{
        create::{CreateRouteCommand, CreateRouteCommandHandler},
        delete::DeleteRouteCommand,
        update::{UpdateRouteCommand, UpdateRouteCommandHandler},
    },
    CommandBus,
};
use perroute_commons::types::actor::Actor;
use perroute_query_bus::QueryBus;
use std::future::Future;

pub trait RouteRestService {
    fn create(
        &self,
        actor: &Actor,
        path: &RouteCollectionPath,
        _req: &CreateRouteRequest,
    ) -> impl Future<Output = ResourceModelResult<RouteModel>>;

    fn update(
        &self,
        actor: &Actor,
        path: &RoutePath,
        _req: &UpdateRouteRequest,
    ) -> impl Future<Output = ResourceModelResult<RouteModel>>;

    fn get(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> impl Future<Output = Option<ResourceModelResult<RouteModel>>>;

    fn query(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> impl Future<Output = ResourceModelCollectionResult<RouteModel>>;

    fn delete(&self, actor: &Actor, path: &RoutePath) -> impl Future<Output = ApiResult<()>>;
}

impl<CB: CommandBus, QB: QueryBus> RouteRestService for RestService<CB, QB> {
    async fn create(
        &self,
        actor: &Actor,
        path: &RouteCollectionPath,
        payload: &CreateRouteRequest,
    ) -> ResourceModelResult<RouteModel> {
        let cmd = CreateRouteCommand::builder()
            .business_id(path.business_unit_id())
            .channel_id(payload.channel_id())
            .message_type_id(payload.message_type_id())
            .configuration(payload.configuration())
            .enabled(payload.enabled())
            .priority(payload.priority())
            .build();

        let route = self
            .command_bus()
            .execute::<_, CreateRouteCommandHandler, _>(actor, &cmd)
            .await?;

        Ok(route.into())
    }

    async fn update(
        &self,
        actor: &Actor,
        path: &RoutePath,
        payload: &UpdateRouteRequest,
    ) -> ResourceModelResult<RouteModel> {
        let cmd = UpdateRouteCommand::builder()
            .id(path.route_id())
            .business_unit_id(path.business_unit_id())
            .configuration(payload.configuration())
            .enabled(payload.enabled())
            .priority(payload.priority())
            .build();
        let route = self
            .command_bus()
            .execute::<_, UpdateRouteCommandHandler, _>(actor, &cmd)
            .await?;

        Ok(route.into())
    }

    async fn get(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> Option<ResourceModelResult<RouteModel>> {
        todo!()
    }

    async fn delete(&self, actor: &Actor, path: &RoutePath) -> ApiResult<()> {
        self.get(actor, path).await;
        let cmd = DeleteRouteCommand::builder().id(path.route_id()).build();

        todo!()
    }

    async fn query(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> ResourceModelCollectionResult<RouteModel> {
        todo!()
    }
}
