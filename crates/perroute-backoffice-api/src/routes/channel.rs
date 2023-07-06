use crate::{
    api::{
        models::channel::{ChannelResource, CreateChannelRequest, UpdateChannelRequest},
        response::{
            ApiResult, CollectionResourceModel, EmptyApiResult, NewApiResponse, SingleResourceModel,
        },
        ResourceLink,
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
};
use actix_web::web;
use anyhow::Context;
use perroute_commons::{
    new_id,
    types::{actor::Actor, id::Id},
};
use perroute_cqrs::{
    command_bus::{
        commands::{
            CreateChannelCommand, CreateChannelCommandBuilder, DeleteChannelCommandBuilder,
            UpdateChannelCommandBuilder,
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

pub type SingleResult = ApiResult<SingleResourceModel<ChannelResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<ChannelResource>>;

pub struct ChannelRouter;

impl ChannelRouter {
    #[tracing::instrument(skip(state))]
    pub async fn create_channel(
        state: web::Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        web::Json(body): web::Json<CreateChannelRequest>,
    ) -> SingleResult {
        let cmd: CreateChannelCommand = CreateChannelCommandBuilder::default()
            .channel_id(new_id!())
            .code(body.code().clone())
            .name(body.name().clone())
            .build()
            .tap_err(|e| tracing::error!("Failed to build CreateChannelCommand: {}", e))
            .with_context(|| "Failed to build CreateChannelCommand")?;

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
        state: web::Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: web::Path<Id>,
    ) -> SingleResult {
        Self::retrieve_channel(state.query_bus(), &actor, path.into_inner(), {
            NewApiResponse::ok
        })
        .await
    }

    #[tracing::instrument(skip(state))]
    pub async fn query_channels(
        state: web::Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> CollectionResult {
        let query = QueryChannelsQueryBuilder::default().build().unwrap();
        state
            .query_bus()
            .execute::<_, QueryChannelsQueryHandler, _>(&actor, &query)
            .await
            .map(NewApiResponse::ok)
            .map_err(ApiError::from)
    }

    #[tracing::instrument(skip(state))]
    pub async fn update_channel(
        state: web::Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: web::Path<Id>,
        web::Json(body): web::Json<UpdateChannelRequest>,
    ) -> SingleResult {
        let channel =
            Self::retrieve_channel(state.query_bus(), &actor, path.into_inner(), identity).await?;

        let cmd = UpdateChannelCommandBuilder::default()
            .channel_id(*channel.id())
            .name(body.name)
            .build()
            .tap_err(|e| tracing::error!("Failed to build UpdateChannelCommand: {}", e))
            .with_context(|| "Failed to build UpdateChannelCommand")?;

        Ok(state
            .command_bus()
            .execute::<_, UpdateChannelCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to update channel: {e}"))
            .map(NewApiResponse::ok)?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn delete_channel(
        state: web::Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: web::Path<Id>,
    ) -> EmptyApiResult {
        let channel =
            Self::retrieve_channel(state.query_bus(), &actor, path.into_inner(), identity).await?;

        let cmd = DeleteChannelCommandBuilder::default()
            .channel_id(*channel.id())
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteChannelCommand: {}", e))
            .with_context(|| "Failed to build DeleteChannelCommand")?;

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
        let query = FindChannelByIdQueryBuilder::default()
            .channel_id(id)
            .build()
            .tap_err(|e| tracing::error!("Failed to build FindChannelByCodeQuery: {}", e))
            .with_context(|| "Failed to build FindChannelByCodeQuery")?;

        query_bus
            .execute::<_, FindChannelByIdHandler, _>(actor, &query)
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel: {e}"))?
            .ok_or_else(|| ApiError::ChannelNotFound(id))
            .map(map)
    }
}
