use super::prelude::*;
use crate::{
    api::{
        models::channel::{ChannelResource, CreateChannelRequest, UpdateChannelRequest},
        ResourceLink,
    },
    extractors::actor::ActorExtractor,
};
use perroute_cqrs::{
    command_bus::handlers::channel::{
        create_channel::CreateChannelCommandHandler, delete_channel::DeleteChannelCommandHandler,
        update_channel::UpdateChannelCommandHandler,
    },
    query_bus::handlers::channel::{
        find_channel_by_id::FindChannelByIdHandler, query_channels::QueryChannelsQueryHandler,
    },
};
use perroute_storage::models::channel::Channel;
use std::convert::identity;

pub const CHANNEL_RESOURCE_NAME: &str = "channel";
pub const CHANNELS_RESOURCE_NAME: &str = "channels";

pub type SingleResult = ApiResult<SingleResourceModel<ChannelResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<ChannelResource>>;

pub struct ChannelRouter;

impl ChannelRouter {
    #[tracing::instrument(skip(state))]
    pub async fn create_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateChannelRequest>,
    ) -> SingleResult {
        let cmd: CreateChannelCommand = CreateChannelCommandBuilder::default()
            .code(body.code().clone())
            .name(body.name().clone())
            .build()
            .tap_err(|e| tracing::error!("Failed to build CreateChannelCommand: {e}"))?;

        Ok(state
            .command_bus()
            .execute::<_, CreateChannelCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to create channel: {e}"))
            .map(|channel| {
                NewApiResponse::created(ResourceLink::Channel(*channel.id()), channel)
            })?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> SingleResult {
        Self::retrieve_channel(state.query_bus(), &actor, path.into_inner(), {
            NewApiResponse::ok
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
            .tap_err(|e| tracing::error!("Failed to build QueryChannelsQuery: {e}"))?;
        state
            .query_bus()
            .execute::<_, QueryChannelsQueryHandler, _>(&actor, &query)
            .await
            .tap_err(|e| tracing::error!("Failed to query channels: {e}"))
            .map(NewApiResponse::ok)
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
            .build()
            .tap_err(|e| tracing::error!("Failed to build UpdateChannelCommand: {e}"))?;

        Ok(state
            .command_bus()
            .execute::<_, UpdateChannelCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to update channel: {e}"))
            .map(NewApiResponse::ok)?)
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
            .tap_err(|e| tracing::error!("Failed to build DeleteChannelCommand: {e}"))?;

        Ok(state
            .command_bus()
            .execute::<_, DeleteChannelCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to delete channel: {e}"))
            .map(|_| NewApiResponse::ok_empty())?)
    }

    pub async fn retrieve_channel<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        id: Id,
        map: impl FnOnce(Channel) -> R,
    ) -> Result<R, ApiError> {
        let query = FindChannelQueryBuilder::default()
            .channel_id(id)
            .build()
            .tap_err(|e| tracing::error!("Failed to build FindChannelByCodeQuery: {e}"))?;

        query_bus
            .execute::<_, FindChannelByIdHandler, _>(actor, &query)
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel: {e}"))?
            .ok_or_else(|| ApiError::ChannelNotFound(id))
            .map(map)
    }
}
