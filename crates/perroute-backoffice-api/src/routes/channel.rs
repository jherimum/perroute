use crate::{
    api::{
        models::channel::{ChannelResource, CreateChannelRequest, UpdateChannelRequest},
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
        types::SingleIdPath,
    },
    app::AppState,
    extractors::actor::ActorExtractor,
    W,
};
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use actix_web_validator::Json;
use anyhow::Context;
use perroute_commandbus::command::channel::{
    create_channel::{CreateChannelCommand, CreateChannelCommandBuilder},
    delete_channel::{DeleteChannelCommand, DeleteChannelCommandBuilder},
    update_channel::{UpdateChannelCommand, UpdateChannelCommandBuilder},
};
use perroute_commons::{new_id, types::id::Id};
use tap::TapFallible;

pub type SingleResult = ApiResult<SingleResourceModel<ChannelResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<ChannelResource>>;

impl TryFrom<CreateChannelRequest> for CreateChannelCommand {
    type Error = anyhow::Error;

    fn try_from(value: CreateChannelRequest) -> Result<Self, Self::Error> {
        Ok(CreateChannelCommandBuilder::default()
            .id(new_id!())
            .business_unit_id(value.into_business_id()?)
            .connection_id(value.into_connection_id()?)
            .dispatch_properties(value.into_properties()?)
            .dispatch_type(value.into_dispatch_type()?)
            .build()?)
    }
}

impl TryFrom<W<(SingleIdPath, UpdateChannelRequest)>> for UpdateChannelCommand {
    type Error = anyhow::Error;
    fn try_from(value: W<(SingleIdPath, UpdateChannelRequest)>) -> Result<Self, Self::Error> {
        Ok(UpdateChannelCommandBuilder::default()
            .id(value.0 .0.try_into().context("context")?)
            .dispatch_properties(value.0 .1.into_properties()?)
            .enabled(value.0 .1.into_enabled())
            .build()?)
    }
}

impl TryFrom<SingleIdPath> for DeleteChannelCommand {
    type Error = anyhow::Error;

    fn try_from(value: SingleIdPath) -> Result<Self, Self::Error> {
        Ok(DeleteChannelCommandBuilder::default()
            .id(value.try_into().context("Invalid id")?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteChannelCommand: {e}"))?)
    }
}

pub struct ChannelsRouter;

impl ChannelsRouter {
    pub const CHANNEL_RESOURCE_NAME: &str = "channel";
    pub const CHANNELS_RESOURCE_NAME: &str = "channels";

    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateChannelRequest>,
    ) -> SingleResult {
        Ok(state
            .command_bus()
            .execute::<CreateChannelCommand, _>(actor, body.try_into()?)
            .await
            .tap_err(|e| tracing::error!("Failed to create channel: {e}"))
            .map(ApiResponse::ok)?)
    }

    pub async fn update(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<UpdateChannelRequest>,
        path: Path<SingleIdPath>,
    ) -> SingleResult {
        Ok(state
            .command_bus()
            .execute::<UpdateChannelCommand, _>(actor, W((path.into_inner(), body)).try_into()?)
            .await
            .tap_err(|e| tracing::error!("Failed to update channel: {e}"))
            .map(ApiResponse::ok)?)
    }

    pub async fn delete(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
    ) -> EmptyApiResult {
        Ok(state
            .command_bus()
            .execute::<DeleteChannelCommand, _>(actor, path.into_inner().try_into()?)
            .await
            .tap_err(|e| tracing::error!("Failed to delete channel: {e}"))
            .map(|_| ApiResponse::ok_empty())?)
    }

    pub async fn get(
        _: Data<AppState>,
        ActorExtractor(_): ActorExtractor,
        _: Path<Id>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    pub async fn query(_: Data<AppState>, ActorExtractor(_): ActorExtractor) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
