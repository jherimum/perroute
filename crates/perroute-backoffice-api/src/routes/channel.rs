use crate::{
    api::{
        models::{
            channel::{ChannelResource, CreateChannelRequest, UpdateChannelRequest},
            SingleIdPath,
        },
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
    },
    app::AppState,
    extractors::actor::ActorExtractor,
    W,
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use anyhow::Context;
use perroute_commons::{new_id, types::id::Id};
use perroute_cqrs::command_bus::handlers::channel::{
    create_channel::{
        CreateChannelCommand, CreateChannelCommandBuilder, CreateChannelCommandHandler,
    },
    delete_channel::{
        DeleteChannelCommand, DeleteChannelCommandBuilder, DeleteChannelCommandHandler,
    },
    update_channel::{
        UpdateChannelCommand, UpdateChannelCommandBuilder, UpdateChannelCommandHandler,
    },
};
use tap::TapFallible;

pub type SingleResult = ApiResult<SingleResourceModel<ChannelResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<ChannelResource>>;

impl TryFrom<CreateChannelRequest> for CreateChannelCommand {
    type Error = anyhow::Error;

    fn try_from(value: CreateChannelRequest) -> Result<Self, Self::Error> {
        Ok(CreateChannelCommandBuilder::default()
            .id(new_id!())
            .business_unit_id(
                value
                    .business_id
                    .context("business id required")?
                    .try_into()
                    .context("Invalid id")?,
            )
            .connection_id(
                value
                    .connection_id
                    .context("connection id required")?
                    .try_into()
                    .context("Invalid id")?,
            )
            .priority(value.priority.context("priority required")?.into())
            .dispatch_properties(value.properties.context("properties required")?.into())
            .dispatch_type(
                value
                    .dispatch_type
                    .context("dispatch type required")?
                    .try_into()
                    .context("Invalid dispatch type")?,
            )
            .build()?)
    }
}

impl TryFrom<W<(SingleIdPath, UpdateChannelRequest)>> for UpdateChannelCommand {
    type Error = anyhow::Error;
    fn try_from(value: W<(SingleIdPath, UpdateChannelRequest)>) -> Result<Self, Self::Error> {
        Ok(UpdateChannelCommandBuilder::default()
            .id(value.0 .0.try_into().context("context")?)
            .priority(value.0 .1.priority.map(Into::into))
            .dispatch_properties(value.0 .1.properties.map(Into::into))
            .enabled(value.0 .1.enabled)
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
            .execute::<_, CreateChannelCommandHandler, _>(&actor, &body.try_into()?)
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
            .execute::<_, UpdateChannelCommandHandler, _>(
                &actor,
                &W((path.into_inner(), body)).try_into()?,
            )
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
            .execute::<_, DeleteChannelCommandHandler, _>(&actor, &path.into_inner().try_into()?)
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
