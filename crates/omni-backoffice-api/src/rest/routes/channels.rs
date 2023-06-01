use crate::rest::api_models::channel::{
    ChannelResource, CreateChannelRequest, UpdateChannelRequest,
};
use crate::rest::error::RestError;
use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use omni_cqrs::commands::channel::{
    create_channel, delete_channel, find_channel, query_channels, update_channel,
};
use omni_cqrs::message_bus::MessageBus;

pub fn routes(message_bus: MessageBus) -> Router {
    Router::new()
        .route("/", get(query))
        .route("/", post(create))
        .route("/:id", get(find))
        .route("/:id", post(update))
        .route("/:id", delete(delete_channel))
        .with_state(message_bus)
}

/* create a axum handler for get */
async fn query(
    State(message_bus): State<MessageBus>,
) -> Result<Json<Vec<ChannelResource>>, RestError> {
    Ok(Json(
        message_bus
            .execute::<query_channels::Handler, _, _, _>(query_channels::Command)
            .await
            .unwrap()
            .unwrap()
            .into_iter()
            .map(ChannelResource::from)
            .collect::<Vec<_>>(),
    ))
}
async fn create(
    State(message_bus): State<MessageBus>,
    Json(req): Json<CreateChannelRequest>,
) -> Result<Json<ChannelResource>, RestError> {
    Ok(Json(
        message_bus
            .execute::<create_channel::Handler, _, _, _>(req.into())
            .await
            .unwrap()
            .unwrap()
            .into(),
    ))
}

impl From<CreateChannelRequest> for create_channel::Command {
    fn from(value: CreateChannelRequest) -> Self {
        create_channel::Command::new(value.code, value.name)
    }
}

async fn find(
    State(message_bus): State<MessageBus>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<ChannelResource>, RestError> {
    Ok(Json(
        message_bus
            .execute::<find_channel::Handler, _, _, _>(find_channel::Command::new(id))
            .await
            .unwrap()
            .unwrap()
            .unwrap()
            .into(),
    ))
}

async fn update(
    State(message_bus): State<MessageBus>,
    Path(id): Path<uuid::Uuid>,
    Json(req): Json<UpdateChannelRequest>,
) -> Result<Json<ChannelResource>, RestError> {
    // Ok(Json(
    //     message_bus
    //         .execute::<update_channel::Handler, _, _, _>((id, req).into())
    //         .await
    //         .unwrap()
    //         .unwrap()
    //         .into(),
    // ))
    todo!()
}

// impl From<(uuid::Uuid, UpdateChannelRequest)> for update_channel::Command {
//     fn from(value: (uuid::Uuid, UpdateChannelRequest)) -> Self {
//         update_channel::Command::new(value.0, value.1.name)
//     }
// }

async fn delete_channel(
    State(message_bus): State<MessageBus>,
    Path(id): Path<uuid::Uuid>,
) -> Result<(), RestError> {
    message_bus
        .execute::<delete_channel::Handler, _, _, _>(delete_channel::Command::new(id))
        .await
        .unwrap()
        .unwrap();
    Ok(())
}
