use crate::{
    api::{
        models::channel::{ChannelResource, CreateChannelRequest, UpdateChannelRequest},
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
};
use actix_web::web::{Data, Json, Path};
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_cqrs::{
    command_bus::handlers::channel::{
        create_channel::{CreateChannelCommandBuilder, CreateChannelCommandHandler},
        delete_channel::{DeleteChannelCommandBuilder, DeleteChannelCommandHandler},
        update_channel::{UpdateChannelCommandBuilder, UpdateChannelCommandHandler},
    },
    query_bus::{
        bus::QueryBus,
        handlers::channel::{
            find_channel::{FindChannelQueryBuilder, FindChannelQueryHandler},
            query_channels::{QueryChannelsQueryBuilder, QueryChannelsQueryHandler},
        },
    },
};
use perroute_storage::models::channel::Channel;
use std::convert::identity;
use tap::TapFallible;

pub type SingleResult = ApiResult<SingleResourceModel<ChannelResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<ChannelResource>>;

pub struct ChannelRouter;

impl ChannelRouter {
    pub const CHANNEL_RESOURCE_NAME: &str = "channel";
    pub const CHANNELS_RESOURCE_NAME: &str = "channels";

    #[tracing::instrument(skip(state))]
    pub async fn create_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateChannelRequest>,
    ) -> SingleResult {
        let cmd = CreateChannelCommandBuilder::default()
            .code(body.code().clone())
            .name(body.name().clone())
            .enabled(body.enabled().clone())
            .vars(body.vars().clone())
            .build()
            .tap_err(|e| tracing::error!("Failed to build CreateChannelCommand: {e}"))
            .map_err(anyhow::Error::from)?;

        Ok(state
            .command_bus()
            .execute::<_, CreateChannelCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to create channel: {e}"))
            .map(|channel| ApiResponse::created(ResourceLink::Channel(*channel.id()), channel))?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> SingleResult {
        Self::retrieve_channel(state.query_bus(), &actor, path.into_inner(), {
            ApiResponse::ok
        })
        .await
    }

    #[tracing::instrument(skip(state))]
    pub async fn query_channels(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> CollectionResult {
        let query = QueryChannelsQueryBuilder::default()
            .build()
            .tap_err(|e| tracing::error!("Failed to build QueryChannelsQuery: {e}"))
            .map_err(anyhow::Error::from)?;
        state
            .query_bus()
            .execute::<_, QueryChannelsQueryHandler, _>(&actor, &query)
            .await
            .tap_err(|e| tracing::error!("Failed to query channels: {e}"))
            .map(ApiResponse::ok)
            .map_err(ApiError::from)
    }

    #[tracing::instrument(skip(state))]
    pub async fn update_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
        Json(body): Json<UpdateChannelRequest>,
    ) -> SingleResult {
        let channel =
            Self::retrieve_channel(state.query_bus(), &actor, path.into_inner(), identity).await?;

        let cmd = UpdateChannelCommandBuilder::default()
            .channel_id(*channel.id())
            .name(body.name)
            .vars(body.vars)
            .enabled(body.enabled)
            .build()
            .tap_err(|e| tracing::error!("Failed to build UpdateChannelCommand: {e}"))
            .map_err(anyhow::Error::from)?;

        Ok(state
            .command_bus()
            .execute::<_, UpdateChannelCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to update channel: {e}"))
            .map(ApiResponse::ok)?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn delete_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> EmptyApiResult {
        let channel =
            Self::retrieve_channel(state.query_bus(), &actor, path.into_inner(), identity).await?;

        let cmd = DeleteChannelCommandBuilder::default()
            .channel_id(*channel.id())
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteChannelCommand: {e}"))
            .map_err(anyhow::Error::from)?;

        Ok(state
            .command_bus()
            .execute::<_, DeleteChannelCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to delete channel: {e}"))
            .map(|_| ApiResponse::ok_empty())?)
    }

    pub async fn retrieve_channel<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        id: Id,
        map: impl FnOnce(Channel) -> R + Send + Sync,
    ) -> Result<R, ApiError> {
        let query = FindChannelQueryBuilder::default()
            .channel_id(Some(id))
            .build()
            .tap_err(|e| tracing::error!("Failed to build FindChannelByCodeQuery: {e}"))
            .map_err(anyhow::Error::from)?;

        query_bus
            .execute::<_, FindChannelQueryHandler, _>(actor, &query)
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel: {e}"))?
            .ok_or_else(|| ApiError::ChannelNotFound(id))
            .map(map)
    }
}
