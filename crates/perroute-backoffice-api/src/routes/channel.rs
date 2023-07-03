use crate::{
    api::{
        models::channel::{ChannelResource, CreateChannelRequest, UpdateChannelRequest},
        response::{ApiResponse, ApiResult, EmptyResource},
        ResourceLink,
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
};
use actix_web::web::{Data, Json, Path};
use anyhow::Context;
use perroute_commons::{
    new_id,
    types::{actor::Actor, id::Id},
};
use perroute_cqrs::{
    command_bus::{
        commands::{
            CreateChannelCommandBuilder, DeleteChannelCommandBuilder, UpdateChannelCommandBuilder,
        },
        handlers::channel::{
            create_channel::CreateChannelCommandHandler,
            delete_channel::DeleteChannelCommandHandler,
            update_channel::UpdateChannelCommandHandler,
        },
    },
    query_bus::{
        bus::QueryBus,
        handlers::channel::{
            find_channel_by_id::FindChannelByIdHandler, query_channels::QueryChannelsQueryHandler,
        },
        queries::{FindChannelByIdQueryBuilder, QueryChannelsQueryBuilder},
    },
};
use perroute_storage::models::channel::Channel;
use std::convert::identity;
use tap::TapFallible;

pub const CHANNEL_RESOURCE_NAME: &str = "channel";
pub const CHANNELS_RESOURCE_NAME: &str = "channels";

pub struct ChannelRouter;

impl ChannelRouter {
    #[tracing::instrument(skip(state))]
    pub async fn create_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateChannelRequest>,
    ) -> ApiResult<ChannelResource> {
        let cmd = CreateChannelCommandBuilder::default()
            .channel_id(new_id!())
            .code(body.code().clone())
            .name(body.name().clone())
            .build()
            .tap_err(|e| tracing::error!("Failed to build CreateChannelCommand: {}", e))
            .with_context(|| "Failed to build CreateChannelCommand")?;

        state
            .command_bus()
            .execute::<_, CreateChannelCommandHandler>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to create channel: {e}"))?;

        Self::retrieve_channel(
            state.query_bus(),
            &actor,
            cmd.channel_id(),
            || ApiError::Const("Failed to retrieve channel"),
            |channel| ApiResponse::Created(ResourceLink::Channel(*channel.id()), channel.into()),
        )
        .await
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> ApiResult<ChannelResource> {
        let channel_id = path.into_inner();
        Self::retrieve_channel(
            state.query_bus(),
            &actor,
            &channel_id,
            || ApiError::ChannelNotFound(channel_id),
            |channel| ApiResponse::OkSingle(channel.into()),
        )
        .await
    }

    #[tracing::instrument(skip(state))]
    pub async fn query_channels(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> ApiResult<ChannelResource> {
        let query = QueryChannelsQueryBuilder::default().build().unwrap();
        state
            .query_bus()
            .execute::<_, QueryChannelsQueryHandler, _>(&actor, &query)
            .await
            .map(|channels| ApiResponse::OkCollection(channels.into()))
            .map_err(ApiError::from)
    }

    #[tracing::instrument(skip(state))]
    pub async fn update_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
        Json(body): Json<UpdateChannelRequest>,
    ) -> ApiResult<ChannelResource> {
        let id = path.into_inner();
        let channel = Self::retrieve_channel(
            state.query_bus(),
            &actor,
            &id,
            || ApiError::ChannelNotFound(id),
            identity,
        )
        .await?;

        let cmd = UpdateChannelCommandBuilder::default()
            .channel_id(*channel.id())
            .name(body.name)
            .build()
            .tap_err(|e| tracing::error!("Failed to build UpdateChannelCommand: {}", e))
            .with_context(|| "Failed to build UpdateChannelCommand")?;

        state
            .command_bus()
            .execute::<_, UpdateChannelCommandHandler>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to update channel: {e}"))?;

        Self::retrieve_channel(
            state.query_bus(),
            &actor,
            &id,
            || ApiError::Const("Failed to retrieve channel"),
            |channel| ApiResponse::OkSingle(channel.into()),
        )
        .await
    }

    #[tracing::instrument(skip(state))]
    pub async fn delete_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> ApiResult<EmptyResource> {
        let channel_id = path.into_inner();
        let channel = Self::retrieve_channel(
            state.query_bus(),
            &actor,
            &channel_id,
            || ApiError::ChannelNotFound(channel_id),
            identity,
        )
        .await?;

        let cmd = DeleteChannelCommandBuilder::default()
            .channel_id(*channel.id())
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteChannelCommand: {}", e))
            .with_context(|| "Failed to build DeleteChannelCommand")?;

        state
            .command_bus()
            .execute::<_, DeleteChannelCommandHandler>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to delete channel: {e}"))
            .map(|_| ApiResponse::OkEmpty(EmptyResource))
            .map_err(ApiError::from)
    }

    pub async fn retrieve_channel<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        id: &Id,
        not_found: impl FnOnce() -> ApiError,
        map: impl FnOnce(Channel) -> R,
    ) -> Result<R, ApiError> {
        let query = FindChannelByIdQueryBuilder::default()
            .channel_id(*id)
            .build()
            .tap_err(|e| tracing::error!("Failed to build FindChannelByCodeQuery: {}", e))
            .with_context(|| "Failed to build FindChannelByCodeQuery")?;

        query_bus
            .execute::<_, FindChannelByIdHandler, _>(actor, &query)
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel: {e}"))?
            .ok_or_else(not_found)
            .map(map)
    }
}
