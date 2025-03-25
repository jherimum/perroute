use super::models::{
    ChannelCollectionPath, ChannelModel, ChannelPath, CreateChannelRequest,
    UpdateChannelRequest,
};
use crate::rest::{
    error::ApiError, modules::business_unit::service::BusinessUnitRestService,
    service::RestService, MaybeResourceModelResult,
    ResourceModelCollectionResult, ResourceModelResult, RestServiceResult,
};
use perroute_command_bus::{
    commands::channel::{
        create::{CreateChannelCommand, CreateChannelCommandHandler},
        delete::{DeleteChannelCommand, DeleteChannelCommandHandler},
        update::{UpdateChannelCommand, UpdateChannelCommandHandler},
    },
    CommandBus,
};
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_query_bus::QueryBus;
use std::future::Future;

pub trait ChannelRestService {
    fn create(
        &self,
        actor: &Actor,
        path: &ChannelCollectionPath,
        _req: &CreateChannelRequest,
    ) -> impl Future<Output = ResourceModelResult<ChannelModel>>;

    fn update(
        &self,
        actor: &Actor,
        path: &ChannelPath,
        _req: &UpdateChannelRequest,
    ) -> impl Future<Output = ResourceModelResult<ChannelModel>>;

    fn delete(
        &self,
        actor: &Actor,
        path: &ChannelPath,
    ) -> impl Future<Output = RestServiceResult<()>>;

    fn get(
        &self,
        actor: &Actor,
        path: &ChannelPath,
    ) -> impl Future<Output = ResourceModelResult<ChannelModel>>;

    fn maybe_get(
        &self,
        actor: &Actor,
        path: &ChannelPath,
    ) -> impl Future<Output = MaybeResourceModelResult<ChannelModel>>;

    fn query(
        &self,
        actor: &Actor,
        path: &ChannelCollectionPath,
    ) -> impl Future<Output = ResourceModelCollectionResult<ChannelModel>>;
}

impl<CB: CommandBus, QB: QueryBus> ChannelRestService for RestService<CB, QB> {
    async fn create(
        &self,
        actor: &Actor,
        path: &ChannelCollectionPath,
        payload: &CreateChannelRequest,
    ) -> ResourceModelResult<ChannelModel> {
        BusinessUnitRestService::get(self, actor, &path.business_unit_path())
            .await?;

        let cmd = CreateChannelCommand::builder()
            .channel_id(Id::new())
            .configuration(payload.configuration())
            .enabled(payload.enabled())
            .name(payload.name()?)
            .business_unit_id(path.business_unit_id())
            .provider_id(payload.provider_id())
            .dispatch_type(payload.dispatch_type()?)
            .build();

        let channel = self
            .command_bus()
            .execute::<_, CreateChannelCommandHandler, _>(actor, &cmd)
            .await?;

        Ok(channel.into())
    }

    async fn update(
        &self,
        actor: &Actor,
        path: &ChannelPath,
        payload: &UpdateChannelRequest,
    ) -> ResourceModelResult<ChannelModel> {
        ChannelRestService::get(self, actor, path).await?;

        let cmd = UpdateChannelCommand::builder()
            .channel_id(path.channel_id())
            .configuration(payload.configuration())
            .enabled(payload.enabled())
            .name(payload.name()?)
            .build();

        let channel = self
            .command_bus()
            .execute::<_, UpdateChannelCommandHandler, _>(actor, &cmd)
            .await?;

        Ok(channel.into())
    }

    async fn delete(
        &self,
        actor: &Actor,
        path: &ChannelPath,
    ) -> RestServiceResult<()> {
        ChannelRestService::get(self, actor, path).await?;
        let cmd = DeleteChannelCommand::builder()
            .channel_id(path.channel_id().clone())
            .build();

        self.command_bus()
            .execute::<_, DeleteChannelCommandHandler, _>(actor, &cmd)
            .await?;

        Ok(())
    }

    async fn query(
        &self,
        actor: &Actor,
        path: &ChannelCollectionPath,
    ) -> ResourceModelCollectionResult<ChannelModel> {
        BusinessUnitRestService::get(self, actor, &path.business_unit_path())
            .await?;
        todo!()
    }

    async fn maybe_get(
        &self,
        actor: &Actor,
        path: &ChannelPath,
    ) -> MaybeResourceModelResult<ChannelModel> {
        todo!()
    }

    async fn get(
        &self,
        actor: &Actor,
        path: &ChannelPath,
    ) -> ResourceModelResult<ChannelModel> {
        self.maybe_get(actor, path).await?.ok_or(ApiError::NotFound)
    }
}
