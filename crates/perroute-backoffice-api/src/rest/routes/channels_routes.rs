use crate::errors::PerrouteBackofficeApiError;
use crate::rest::api_models::channel::{
    ChannelResource, CreateChannelRequest, UpdateChannelRequest,
};
use crate::rest::extractors::actor::ActorExtractor;
use crate::rest::Buses;
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::routing::{get, put};
use axum::{Json, Router};
use perroute_commons::new_id;
use perroute_commons::rest::RestError;
use perroute_commons::types::actor::Actor;
use perroute_commons::types::id::{self, Id};
use perroute_cqrs::command_bus::bus::CommandBus;

use perroute_cqrs::command_bus::commands::{
    CreateChannelCommandBuilder, DeleteChannelCommandBuilder, UpdateChannelCommandBuilder,
};
use perroute_cqrs::command_bus::handlers::channel::create_channel::CreateChannelCommandHandler;
use perroute_cqrs::command_bus::handlers::channel::delete_channel::DeleteChannelCommandHandler;
use perroute_cqrs::command_bus::handlers::channel::update_channel::UpdateChannelCommandHandler;
use perroute_cqrs::query_bus::bus::QueryBus;
use perroute_cqrs::query_bus::handlers::channel::find_channel::FindChannelQueryHandler;
use perroute_cqrs::query_bus::handlers::channel::query_channels::QueryChannelsQueryHandler;
use perroute_cqrs::query_bus::queries::{FindChannelQueryBuilder, QueryChannelsQueryBuilder};
use tap::{TapFallible, TapOptional};

pub fn routes(buses: Buses) -> Router {
    Router::new().nest(
        "/v1/channels",
        Router::new()
            .route("/", get(query_channels))
            .route("/", post(create_channel))
            .nest(
                "/:channel_id",
                Router::new()
                    .route("/", get(find_channel))
                    .route("/", put(update_channel))
                    .route("/", delete(delete_channel)),
            )
            .with_state(buses),
    )
}

async fn retrieve_channel_resource(
    actor: &Actor,
    query_bus: &QueryBus,
    channel_id: Id,
    not_found: impl FnOnce(Id) -> RestError,
) -> Result<Json<ChannelResource>, RestError> {
    let query = FindChannelQueryBuilder::default()
        .channel_id(channel_id)
        .build()
        .tap_err(|e| tracing::error!("Failed to build FindChannelQueryBuilder: {e}"))
        .map_err(|_| RestError::InternalServer)?;

    query_bus
        .execute::<_, FindChannelQueryHandler, _>(actor, query)
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve channel: {e}"))
        .map_err(PerrouteBackofficeApiError::from)?
        .map(ChannelResource::from)
        .map(Json::from)
        .tap_none(|| tracing::error!("Channel {channel_id} not found"))
        .ok_or(not_found(channel_id))
}

#[tracing::instrument(skip(command_bus, query_bus))]
async fn create_channel(
    State(command_bus): State<CommandBus>,
    State(query_bus): State<QueryBus>,
    ActorExtractor(actor): ActorExtractor,
    Json(body): Json<CreateChannelRequest>,
) -> Result<Json<ChannelResource>, RestError> {
    let command = CreateChannelCommandBuilder::default()
        .channel_id(new_id!())
        .code(body.code)
        .name(body.name)
        .build()
        .tap_err(|e| tracing::error!("Failed to build CreateChannelCommandBuilder: {e}"))
        .map_err(|_| RestError::InternalServer)?;

    command_bus
        .execute::<_, CreateChannelCommandHandler>(&actor, command.clone())
        .await
        .tap_err(|e| tracing::error!("Failed to create channel: {e}"))
        .map_err(PerrouteBackofficeApiError::from)?;

    retrieve_channel_resource(&actor, &query_bus, *command.channel_id(), |_| {
        RestError::InternalServer
    })
    .await
}

#[tracing::instrument(skip(query_bus))]
async fn find_channel(
    State(query_bus): State<QueryBus>,
    ActorExtractor(actor): ActorExtractor,
    Path(channel_id): Path<Id>,
) -> Result<Json<ChannelResource>, RestError> {
    retrieve_channel_resource(&actor, &query_bus, channel_id, |id| {
        RestError::NotFound(format!("Channel {id} not found"))
    })
    .await
    .tap_err(|e| tracing::error!("Failed to find channel: {e}"))
}

#[tracing::instrument(skip(query_bus, command_bus))]
async fn update_channel(
    State(query_bus): State<QueryBus>,
    State(command_bus): State<CommandBus>,
    ActorExtractor(actor): ActorExtractor,
    Path(channel_id): Path<Id>,
    Json(req): Json<UpdateChannelRequest>,
) -> Result<Json<ChannelResource>, RestError> {
    let command = UpdateChannelCommandBuilder::default()
        .channel_id(channel_id)
        .name(req.name)
        .build()
        .tap_err(|e| tracing::error!("Failed to build UpdateChannelCommandBuilder: {e}"))
        .map_err(|_| RestError::InternalServer)?;

    command_bus
        .execute::<_, UpdateChannelCommandHandler>(&actor, command)
        .await
        .tap_err(|e| tracing::error!("Failed to update channel: {e}"))
        .map_err(PerrouteBackofficeApiError::from)?;

    retrieve_channel_resource(&actor, &query_bus, channel_id, |id| {
        RestError::NotFound(format!("Channel {id} not found"))
    })
    .await
    .tap_err(|e| tracing::error!("Failed to find channel: {e}"))
}

#[tracing::instrument(skip(command_bus))]
async fn delete_channel(
    State(command_bus): State<CommandBus>,
    ActorExtractor(actor): ActorExtractor,
    Path(id): Path<id::Id>,
) -> Result<(), RestError> {
    let cmd = DeleteChannelCommandBuilder::default()
        .channel_id(id)
        .build()
        .tap_err(|e| tracing::error!("Failed to build DeleteChannelCommandBuilder: {e}"))
        .map_err(|_| RestError::InternalServer)?;

    Ok(command_bus
        .execute::<_, DeleteChannelCommandHandler>(&actor, cmd)
        .await
        .tap_err(|e| tracing::error!("Failed to delete channel: {e}"))
        .map_err(PerrouteBackofficeApiError::from)?)
}

#[tracing::instrument(skip(query_bus))]
async fn query_channels(
    State(query_bus): State<QueryBus>,
    ActorExtractor(actor): ActorExtractor,
) -> Result<Json<Vec<ChannelResource>>, RestError> {
    let query = QueryChannelsQueryBuilder::default().build().unwrap();
    Ok(Json::from(
        query_bus
            .execute::<_, QueryChannelsQueryHandler, _>(&actor, query)
            .await
            .map_err(PerrouteBackofficeApiError::from)?
            .into_iter()
            .map(ChannelResource::from)
            .collect::<Vec<_>>(),
    ))
}
