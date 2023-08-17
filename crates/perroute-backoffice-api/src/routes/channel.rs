use crate::{
    api::{
        models::channel::{ChannelResource, CreateChannelRequest, UpdateChannelRequest},
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
    },
    app::AppState,
    extractors::actor::ActorExtractor,
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use perroute_commons::{new_id, types::id::Id};
use perroute_cqrs::command_bus::handlers::channel::{
    create_channel::{CreateChannelCommandBuilder, CreateChannelCommandHandler},
    delete_channel::{DeleteChannelCommandBuilder, DeleteChannelCommandHandler},
    update_channel::{UpdateChannelCommandBuilder, UpdateChannelCommandHandler},
};
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
        Ok(state
            .command_bus()
            .execute::<_, CreateChannelCommandHandler, _>(
                &actor,
                &CreateChannelCommandBuilder::default()
                    .id(new_id!())
                    .business_unit_id(*body.business_id())
                    .connection_id(*body.connection_id())
                    .priority(*body.priority())
                    .dispatch_properties(body.properties().clone())
                    .dispatch_type(*body.dispatch_type())
                    .build()
                    .unwrap(),
            )
            .await
            .tap_err(|e| tracing::error!("Failed to create channel: {e}"))
            .map(ApiResponse::ok)?)
    }

    pub async fn update(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<UpdateChannelRequest>,
        path: Path<Id>,
    ) -> SingleResult {
        Ok(state
            .command_bus()
            .execute::<_, UpdateChannelCommandHandler, _>(
                &actor,
                &UpdateChannelCommandBuilder::default()
                    .id(path.into_inner())
                    .priority(*body.priority())
                    .dispatch_properties(body.properties().clone())
                    .enabled(*body.enabled())
                    .build()
                    .unwrap(),
            )
            .await
            .tap_err(|e| tracing::error!("Failed to update channel: {e}"))
            .map(ApiResponse::ok)?)
    }

    pub async fn delete(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> EmptyApiResult {
        Ok(state
            .command_bus()
            .execute::<_, DeleteChannelCommandHandler, _>(
                &actor,
                &DeleteChannelCommandBuilder::default()
                    .id(path.into_inner())
                    .build()
                    .unwrap(),
            )
            .await
            .tap_err(|e| tracing::error!("Failed to delete channel: {e}"))
            .map(|_| ApiResponse::ok_empty())?)
    }

    pub async fn get(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    pub async fn query(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
