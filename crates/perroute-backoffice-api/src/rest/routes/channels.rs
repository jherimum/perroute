use crate::rest::api_models::channel::{
    self, ChannelResource, CreateChannelRequest, UpdateChannelRequest,
};
use crate::rest::Buses;

use axum::extract::{Path, State};
use axum::routing::post;
use axum::routing::{get, put};
use axum::{Json, Router};
use perroute_commons::new_id;
use perroute_commons::rest::RestError;
use perroute_commons::types::actor::Actor;
use perroute_commons::types::id::Id;
use perroute_cqrs::command_bus::bus::CommandBus;
use perroute_cqrs::command_bus::commands::channel::create_channel::{
    CreateChannelCommand, CreateChannelCommandHandler,
};
use perroute_cqrs::command_bus::commands::channel::update_channel::{
    UpdateChannelCommand, UpdateChannelCommandHandler,
};
use perroute_cqrs::query_bus::bus::QueryBus;
use perroute_cqrs::query_bus::queries::channel::find_channel::{
    FindChannelQuery, FindChannelQueryHandler,
};
use tap::{TapFallible, TapOptional};

pub fn routes(buses: Buses) -> Router {
    Router::new()
        //.route("/", get(query))
        .route("/", post(create_channel))
        .route("/:id", get(find_channel))
        .route("/:id", put(update_channel))
        //.route("/:id", delete(delete_channel))
        .with_state(buses)
}

async fn retrieve_channel_resource(
    actor: &Actor,
    query_bus: &QueryBus,
    channel_id: Id,
    not_found: impl FnOnce(Id) -> RestError,
) -> Result<Json<ChannelResource>, RestError> {
    query_bus
        .execute::<FindChannelQueryHandler, _, _>(actor.clone(), FindChannelQuery::new(channel_id))
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve channel: {e}"))
        .map_err(|_| RestError::InternalServer)?
        .map(ChannelResource::from)
        .map(Json::from)
        .tap_none(|| tracing::error!("Channel not found"))
        .ok_or(not_found(channel_id))
}

#[tracing::instrument(skip(command_bus, query_bus))]
async fn create_channel(
    State(command_bus): State<CommandBus>,
    State(query_bus): State<QueryBus>,
    Json(body): Json<CreateChannelRequest>,
) -> Result<Json<ChannelResource>, RestError> {
    let actor = Actor::system();
    let command = CreateChannelCommand::new(new_id!(), body.code, body.name);

    command_bus
        .execute::<_, CreateChannelCommandHandler>(actor.clone(), command.clone())
        .await
        .tap_err(|e| tracing::error!("Failed to create channel: {e}"))
        .map_err(|e| RestError::UnprocessableEntity(e.to_string()))?;

    retrieve_channel_resource(&actor, &query_bus, command.channel_id, |_| {
        RestError::InternalServer
    })
    .await
}

#[tracing::instrument(skip(query_bus))]
async fn find_channel(
    State(query_bus): State<QueryBus>,
    Path(channel_id): Path<Id>,
) -> Result<Json<ChannelResource>, RestError> {
    retrieve_channel_resource(&Actor::system(), &query_bus, channel_id, |id| {
        RestError::NotFound(format!("Channel {id} not found"))
    })
    .await
}

#[tracing::instrument(skip(query_bus, command_bus))]
async fn update_channel(
    State(query_bus): State<QueryBus>,
    State(command_bus): State<CommandBus>,
    Path(channel_id): Path<Id>,
    Json(req): Json<UpdateChannelRequest>,
) -> Result<Json<ChannelResource>, RestError> {
    let actor = Actor::system();
    let command = UpdateChannelCommand::new(channel_id, req.name);

    command_bus
        .execute::<_, UpdateChannelCommandHandler>(actor.clone(), command)
        .await
        .unwrap();

    retrieve_channel_resource(&actor, &query_bus, channel_id, |id| {
        RestError::NotFound(format!("Channel {id} not found"))
    })
    .await
}

// /* create a axum handler for get */
// async fn query(
//     State(message_bus): State<MessageBus>,
// ) -> Result<Json<Vec<ChannelResource>>, RestError> {
//     Ok(Json(
//         message_bus
//             .execute::<query_channels::Handler, _, _, _>(Actor::System, query_channels::Command)
//             .await
//             .unwrap()
//             .unwrap()
//             .into_iter()
//             .map(ChannelResource::from)
//             .collect::<Vec<_>>(),
//     ))
// }

// impl From<CreateChannelRequest> for create_channel::CreateChannelCommand {
//     fn from(value: CreateChannelRequest) -> Self {
//         create_channel::CreateChannelCommand::new(value.code, value.name)
//     }
// }

// async fn update(
//     State(message_bus): State<MessageBus>,
//     Path(id): Path<Id>,
//     Json(req): Json<UpdateChannelRequest>,
// ) -> Result<Json<ChannelResource>, RestError> {
//     Ok(Json(
//         message_bus
//             .execute::<update_channel::Handler, _, _, _>(Actor::System, W((id, req)).into())
//             .await
//             .unwrap()
//             .unwrap()
//             .into(),
//     ))
// }

// impl From<W<(id::Id, UpdateChannelRequest)>> for update_channel::Command {
//     fn from(value: W<(id::Id, UpdateChannelRequest)>) -> Self {
//         update_channel::Command::new(value.0 .0, value.0 .1.name)
//     }
// }

// async fn delete_channel(
//     State(message_bus): State<MessageBus>,
//     Path(id): Path<id::Id>,
// ) -> Result<(), RestError> {
//     message_bus
//         .execute::<delete_channel::Handler, _, _, _>(
//             Actor::System,
//             delete_channel::Command::new(id),
//         )
//         .await
//         .unwrap()
//         .unwrap();
//     Ok(())
// }
