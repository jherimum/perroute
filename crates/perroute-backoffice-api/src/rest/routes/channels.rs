use crate::rest::api_models::channel::{ChannelResource, CreateChannelRequest};
use crate::rest::Buses;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::routing::post;
use axum::{Json, Router};
use perroute_commons::new_id;
use perroute_commons::rest::RestError;
use perroute_commons::types::actor::Actor;
use perroute_commons::types::id::Id;
use perroute_cqrs::command_bus::bus::CommandBus;
use perroute_cqrs::command_bus::commands::channel::create_channel::{
    CreateChannelCommand, CreateChannelCommandHandler,
};
use perroute_cqrs::query_bus::bus::QueryBus;
use perroute_cqrs::query_bus::queries::channel::find_channel::{
    FindChannelQuery, FindChannelQueryHandler,
};

//use perroute_cqrs::query_bus::MessageBus;

pub fn routes(buses: Buses) -> Router {
    Router::new()
        //.route("/", get(query))
        .route("/", post(create))
        .route("/:id", get(find))
        //.route("/:id", post(update))
        //.route("/:id", delete(delete_channel))
        .with_state(buses)
}

async fn create(
    State(command_bus): State<CommandBus>,
    State(query_bus): State<QueryBus>,
    Json(body): Json<CreateChannelRequest>,
) -> Result<Json<ChannelResource>, RestError> {
    let actor = Actor::system();
    let command = CreateChannelCommand::new(new_id!(), body.code, body.name);
    command_bus
        .execute::<_, CreateChannelCommandHandler>(actor.clone(), command.clone())
        .await
        .unwrap();
    query_bus
        .execute::<FindChannelQueryHandler, _, _>(actor, FindChannelQuery::new(command.channel_id))
        .await
        .map_err(|_| RestError::InernalServer)?
        .map(ChannelResource::from)
        .map(Json::from)
        .ok_or(RestError::InernalServer)
}

async fn find(
    State(query_bus): State<QueryBus>,
    Path(id): Path<Id>,
) -> Result<Json<ChannelResource>, RestError> {
    Ok(Json(ChannelResource::from(
        query_bus
            .execute::<FindChannelQueryHandler, _, _>(Actor::system(), FindChannelQuery::new(id))
            .await
            .unwrap()
            .unwrap(),
    )))
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

// async fn find(
//     State(message_bus): State<MessageBus>,
//     Path(id): Path<id::Id>,
// ) -> Result<Json<ChannelResource>, RestError> {
//     Ok(Json(
//         message_bus
//             .execute::<find_channel::Handler, _, _, _>(
//                 Actor::System,
//                 find_channel::Command::new(id),
//             )
//             .await
//             .unwrap()
//             .unwrap()
//             .unwrap()
//             .into(),
//     ))
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
