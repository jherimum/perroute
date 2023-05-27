use crate::{
    cqrs::{
        commands::connection::{
            create_connection, delete_connection, find_all_connections, find_connection,
            update_connection,
        },
        message_bus::MessageBus,
    },
    errors::OmniMessageError,
    rest::{
        api_models::connection::{
            ConnectionResource, CreateConnectionRequest, UpdateConnectionRequest,
        },
        error::RestError,
    },
    types::OmniResult,
};
use axum::{extract::Path, routing::delete};
use axum::{
    extract::State,
    routing::{get, patch, post},
    Json, Router,
};
use std::todo;
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
) -> OmniResult<Json<Vec<ConnectionResource>>> {
    Ok(Json::from(
        message_bus
            .execute::<find_all_connections::Handler, _, _>(find_all_connections::Query {})
            .await?
            .into_iter()
            .map(ConnectionResource::from)
            .collect::<Vec<_>>(),
    ))
}

async fn create_connection(
    State(message_bus): State<MessageBus>,
    Json(req): Json<CreateConnectionRequest>,
) -> OmniResult<Json<ConnectionResource>> {
    message_bus
        .execute::<create_connection::CommandHandler, _, _>(create_connection::Command::from(req))
        .await
        .map(ConnectionResource::from)
        .map(Json::from)
}

async fn get_connection(
    State(message_bus): State<MessageBus>,
    Path(connection_id): Path<uuid::Uuid>,
) -> OmniResult<Json<ConnectionResource>> {
    message_bus
        .execute::<find_connection::Handler, _, _>(find_connection::Query(connection_id))
        .await?
        .ok_or(RestError::NotFound("".to_owned()))
        .map_err(OmniMessageError::from)
        .map(ConnectionResource::from)
        .map(Json::from)
}

async fn update_connection(
    State(message_bus): State<MessageBus>,
    Path(connection_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateConnectionRequest>,
) -> OmniResult<Json<ConnectionResource>> {
    message_bus
        .execute::<update_connection::Handler, _, _>(update_connection::Command {
            id: connection_id,
            description: req.description,
            properties: req.properties,
        })
        .await
        .map(ConnectionResource::from)
        .map(Json::from)
}

async fn delete_connection(
    State(message_bus): State<MessageBus>,
    Path(connection_id): Path<uuid::Uuid>,
) -> OmniResult<()> {
    message_bus
        .execute::<delete_connection::Handler, _, _>(delete_connection::Command {
            id: connection_id,
        })
        .await
}

impl From<create_connection::Error> for RestError {
    fn from(value: create_connection::Error) -> Self {
        match value {
            create_connection::Error::Database(_) => RestError::InernalServer,
            create_connection::Error::PluginNotFound(_) => RestError::InernalServer,
            create_connection::Error::ConnectorCodeAlreadyExists(_) => todo!(),
        }
    }
}

impl From<CreateConnectionRequest> for create_connection::Command {
    fn from(value: CreateConnectionRequest) -> Self {
        create_connection::Command {
            code: value.code,
            plugin_id: value.plugin_id,
            properties: value.properties,
            description: value.description,
        }
    }
}
