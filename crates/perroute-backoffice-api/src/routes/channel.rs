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
};
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use actix_web_validator::Json;
use perroute_commandbus::command::channel::{
    create_channel::{CreateChannelCommand, CreateChannelCommandBuilder},
    delete_channel::{DeleteChannelCommand, DeleteChannelCommandBuilder},
    update_channel::{UpdateChannelCommand, UpdateChannelCommandBuilder},
};
use perroute_commons::{new_id, types::id::Id};
use tap::TapFallible;

pub type SingleResult = ApiResult<SingleResourceModel<ChannelResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<ChannelResource>>;

pub struct ChannelsRouter;

impl ChannelsRouter {
    pub const CHANNEL_RESOURCE_NAME: &str = "channel";
    pub const CHANNELS_RESOURCE_NAME: &str = "channels";

    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateChannelRequest>,
    ) -> SingleResult {
        let command = CreateChannelCommandBuilder::default()
            .id(new_id!())
            .business_unit_id(body.business_id()?)
            .connection_id(body.connection_id()?)
            .dispatch_properties(body.properties()?)
            .dispatch_type(body.dispatch_type()?)
            .build()
            .map_err(anyhow::Error::new)?;

        Ok(state
            .command_bus()
            .execute::<CreateChannelCommand, _>(actor, command)
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
        let command = UpdateChannelCommandBuilder::default()
            .id(path.into_inner().try_into()?)
            .dispatch_properties(body.properties()?)
            .enabled(body.enabled())
            .build()
            .map_err(anyhow::Error::new)?;

        Ok(state
            .command_bus()
            .execute::<UpdateChannelCommand, _>(actor, command)
            .await
            .tap_err(|e| tracing::error!("Failed to update channel: {e}"))
            .map(ApiResponse::ok)?)
    }

    pub async fn delete(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
    ) -> EmptyApiResult {
        let command = DeleteChannelCommandBuilder::default()
            .id(path.into_inner().try_into()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteChannelCommand: {e}"))
            .map_err(anyhow::Error::new)?;
        Ok(state
            .command_bus()
            .execute::<DeleteChannelCommand, _>(actor, command)
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
