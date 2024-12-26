use super::models::{
    CreateRouteRequest, RouteCollectionPath, RouteModel, RoutePath,
    UpdateRouteRequest,
};
use crate::rest::{
    error::ApiError, modules::business_unit::service::BusinessUnitRestService,
    service::RestService, MaybeResourceModelResult,
    ResourceModelCollectionResult, ResourceModelResult, RestServiceResult,
};
use perroute_command_bus::{
    commands::route::{
        create::{CreateRouteCommand, CreateRouteCommandHandler},
        delete::{DeleteRouteCommand, DeleteRouteCommandHandler},
        update::{UpdateRouteCommand, UpdateRouteCommandHandler},
    },
    CommandBus,
};
use perroute_commons::types::{actor::Actor, id::Id};
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

    fn delete(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> impl Future<Output = RestServiceResult<()>>;

    fn get(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> impl Future<Output = ResourceModelResult<RouteModel>>;

    fn maybe_get(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> impl Future<Output = MaybeResourceModelResult<RouteModel>>;

    fn query(
        &self,
        actor: &Actor,
        path: &RouteCollectionPath,
    ) -> impl Future<Output = ResourceModelCollectionResult<RouteModel>>;
}

impl<CB: CommandBus, QB: QueryBus> RouteRestService for RestService<CB, QB> {
    async fn create(
        &self,
        actor: &Actor,
        path: &RouteCollectionPath,
        payload: &CreateRouteRequest,
    ) -> ResourceModelResult<RouteModel> {
        BusinessUnitRestService::get(self, actor, &path.business_unit_path())
            .await?;

        let cmd = CreateRouteCommand::builder()
            .route_id(Id::new())
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
        RouteRestService::get(self, actor, path).await?;

        let cmd = UpdateRouteCommand::builder()
            .route_id(path.route_id())
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

    async fn delete(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> RestServiceResult<()> {
        RouteRestService::get(self, actor, path).await?;
        let cmd = DeleteRouteCommand::builder()
            .route_id(path.route_id().clone())
            .build();

        self.command_bus()
            .execute::<_, DeleteRouteCommandHandler, _>(actor, &cmd)
            .await?;

        Ok(())
    }

    async fn query(
        &self,
        actor: &Actor,
        path: &RouteCollectionPath,
    ) -> ResourceModelCollectionResult<RouteModel> {
        BusinessUnitRestService::get(self, actor, &path.business_unit_path())
            .await?;
        todo!()
    }

    async fn maybe_get(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> MaybeResourceModelResult<RouteModel> {
        todo!()
    }

    async fn get(
        &self,
        actor: &Actor,
        path: &RoutePath,
    ) -> ResourceModelResult<RouteModel> {
        self.maybe_get(actor, path).await?.ok_or(ApiError::NotFound)
    }
}
