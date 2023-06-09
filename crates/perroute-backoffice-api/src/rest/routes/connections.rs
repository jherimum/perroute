use axum::{extract::Path, routing::delete};
use axum::{
    extract::State,
    routing::{get, patch, post},
    Json, Router,
};
use perroute_cqrs::commands::connection::{
    create_connection, delete_connection, find_all_connections, find_connection, update_connection,
};
use perroute_cqrs::message_bus::MessageBus;
use std::todo;

use crate::rest::api_models::connection::{
    ConnectionResource, CreateConnectionRequest, UpdateConnectionRequest,
};
use crate::types::W;
use perroute_commons::rest::RestError;

pub fn routes(message_bus: MessageBus) -> Router {
    Router::new()
        .route("/", get(get_all_connections))
        .route("/", post(create_connection))
        .route("/:id", get(get_connection))
        .route("/:id", patch(update_connection))
        .route("/:id", delete(delete_connection))
        .with_state(message_bus)
}

async fn get_all_connections(
    State(message_bus): State<MessageBus>,
) -> Result<Json<Vec<ConnectionResource>>, RestError> {
    Ok(message_bus
        .execute::<find_all_connections::Handler, _, _, _>(find_all_connections::Query {})
        .await
        .unwrap()
        .unwrap()
        .into_iter()
        .map(ConnectionResource::from)
        .collect::<Vec<_>>()
        .into())
}

async fn create_connection(
    State(message_bus): State<MessageBus>,
    Json(req): Json<CreateConnectionRequest>,
) -> Result<Json<ConnectionResource>, RestError> {
    Ok(Json(
        message_bus
            .execute::<create_connection::CommandHandler, _, _, _>(
                create_connection::Command::from(req),
            )
            .await
            .unwrap()
            .unwrap()
            .into(),
    ))
}

async fn get_connection(
    State(message_bus): State<MessageBus>,
    Path(connection_id): Path<uuid::Uuid>,
) -> Result<Json<ConnectionResource>, RestError> {
    Ok(Json(
        message_bus
            .execute::<find_connection::Handler, _, _, _>(find_connection::Query(connection_id))
            .await
            .unwrap()
            .unwrap()
            .unwrap()
            .into(),
    ))
}

async fn update_connection(
    State(message_bus): State<MessageBus>,
    Path(connection_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateConnectionRequest>,
) -> Result<Json<ConnectionResource>, RestError> {
    Ok(Json(
        message_bus
            .execute::<update_connection::Handler, _, _, _>(update_connection::Command {
                id: connection_id,
                description: req.description,
                properties: req.properties,
            })
            .await
            .unwrap()
            .unwrap()
            .into(),
    ))
}

async fn delete_connection(
    State(message_bus): State<MessageBus>,
    Path(connection_id): Path<uuid::Uuid>,
) -> Result<(), RestError> {
    message_bus
        .execute::<delete_connection::Handler, _, _, _>(delete_connection::Command {
            id: connection_id,
        })
        .await
        .unwrap()
        .unwrap();
    Ok(())
}

impl From<create_connection::Error> for W<RestError> {
    fn from(_value: create_connection::Error) -> Self {
        // match value {
        //     create_connection::Error::Database(_) => RestError::InernalServer,
        //     create_connection::Error::PluginNotFound(_) => RestError::InernalServer,
        //     create_connection::Error::ConnectorCodeAlreadyExists(_) => todo!(),
        // }
        todo!()
    }
}

impl From<CreateConnectionRequest> for create_connection::Command {
    fn from(_value: CreateConnectionRequest) -> Self {
        // create_connection::Command {
        //     code: value.code,
        //     plugin_id: value.plugin_id,
        //     properties: value.properties,
        //     description: value.description,
        // }
        todo!()
    }
}
