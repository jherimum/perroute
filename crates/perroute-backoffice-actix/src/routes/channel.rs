use crate::{
    api::{ApiResponse, ApiResult, ResourceLink},
    api_models::channel::{ChannelResource, CreateChannelRequest, UpdateChannelRequest},
    error::ApiError,
    extractors::actor::ActorExtractor,
    AppState,
};
use actix_web::{
    web::{delete, get, post, put, resource, Data, Json, Path},
    Scope,
};
use anyhow::Context;
use perroute_commons::{
    new_id,
    types::{actor::Actor, code::Code},
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
            find_channel_by_code::FindChannelByCodeHandler,
            query_channels::QueryChannelsQueryHandler,
        },
        queries::{FindChannelByCodeQueryBuilder, QueryChannelsQueryBuilder},
    },
};
use perroute_storage::models::channel::Channel;
use tap::TapFallible;

pub const CHANNEL_RESOUCE_LINK: &str = "channel";
pub const CHANNELS_RESOUCE_LINK: &str = "channels";

pub struct ChannelRouter;

impl ChannelRouter {
    pub fn routes() -> Scope {
        Scope::new("/v1/channels")
            .service(
                resource("")
                    .name(CHANNELS_RESOUCE_LINK)
                    .route(post().to(Self::create_channel))
                    .route(get().to(Self::query)),
            )
            .service(
                resource("/{channel_code}")
                    .name(CHANNEL_RESOUCE_LINK)
                    .route(get().to(Self::find_channel))
                    .route(put().to(Self::update))
                    .route(delete().to(Self::delete)),
            )
    }

    #[tracing::instrument]
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

        let channel = retrieve_channel(state.query_bus(), &actor, body.code(), || {
            ApiError::Const("Failed to retrieve channel")
        })
        .await?;

        Ok(ApiResponse::Created(
            ResourceLink::Channel(channel.code().clone()),
            channel.into(),
        ))
    }

    #[tracing::instrument]
    pub async fn find_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Code>,
    ) -> ApiResult<ChannelResource> {
        let channel_code = path.into_inner();
        let channel = retrieve_channel(state.query_bus(), &actor, &channel_code, || {
            ApiError::ChannelNotFound(channel_code.clone())
        })
        .await?;
        Ok(ApiResponse::OkSingle(channel.into()))
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn query(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> ApiResult<ChannelResource> {
        let query = QueryChannelsQueryBuilder::default().build().unwrap();
        let channels = state
            .query_bus()
            .execute::<_, QueryChannelsQueryHandler, _>(&actor, &query)
            .await?;
        Ok(ApiResponse::OkCollection(channels.into()))
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn update(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Code>,
        Json(body): Json<UpdateChannelRequest>,
    ) -> ApiResult<ChannelResource> {
        let code = path.into_inner();
        let channel = retrieve_channel(state.query_bus(), &actor, &code, || {
            ApiError::ChannelNotFound(code.clone())
        })
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

        let channel = retrieve_channel(state.query_bus(), &actor, &code, || {
            ApiError::Const("Failed to retrieve channel")
        })
        .await?;

        Ok(ApiResponse::OkSingle(channel.into()))
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn delete(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Code>,
    ) -> ApiResult<()> {
        let channel_code = path.into_inner();
        let channel = retrieve_channel(state.query_bus(), &actor, &channel_code, || {
            ApiError::ChannelNotFound(channel_code.clone())
        })
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
            .tap_err(|e| tracing::error!("Failed to delete channel: {e}"))?;

        Ok(ApiResponse::OkEmpty)
    }
}

async fn retrieve_channel(
    query_bus: &QueryBus,
    actor: &Actor,
    code: &Code,
    not_found: impl FnOnce() -> ApiError,
) -> Result<Channel, ApiError> {
    let query = FindChannelByCodeQueryBuilder::default()
        .channel_code(code.clone())
        .build()
        .tap_err(|e| tracing::error!("Failed to build FindChannelByCodeQuery: {}", e))
        .with_context(|| "Failed to build FindChannelByCodeQuery")?;

    query_bus
        .execute::<_, FindChannelByCodeHandler, _>(actor, &query)
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve channel: {e}"))?
        .ok_or_else(not_found)
}
