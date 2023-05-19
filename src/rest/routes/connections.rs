use crate::{
    cqrs::{
        commands::connection::{
            create_connection, delete_connection, find_all_connections, find_connection,
            update_connection,
        },
        message_bus::MessageBus,
    },
    database_models::account::Account,
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
        .route("/", post(create_connection))
        .route("/:id", patch(update_connection))
        .route("/:id", delete(delete_connection))
        .route("/:id", get(get_connection))
        .route("/", get(get_all_connections))
        .with_state(message_bus)
}

impl From<find_all_connections::Error> for RestError {
    fn from(value: find_all_connections::Error) -> Self {
        todo!()
    }
}

async fn get_all_connections(
    account: Account,
    State(message_bus): State<MessageBus>,
) -> OmniResult<Json<Vec<ConnectionResource>>> {
    // message_bus
    //     .execute::<find_all_connections::Handler, _, _>(find_all_connections::Query { account })
    //     .await?
    //     .map(|v| Json(v.into_iter().map(ConnectionResource::from).collect()))
    todo!()
}

impl From<find_connection::Error> for RestError {
    fn from(value: find_connection::Error) -> Self {
        todo!()
    }
}

async fn get_connection(
    account: Account,
    State(message_bus): State<MessageBus>,
    Path(connection_id): Path<uuid::Uuid>,
) -> OmniResult<Json<ConnectionResource>> {
    //message_bus
    // .execute::<find_connection::Handler, _, _, _>(find_connection::Query {
    //     id: connection_id,
    //     account,
    // })
    // .await??
    // .ok_or(RestError::NotFound("()".to_owned()))
    // .map(ConnectionResource::from)
    // .map(Json::from)
    todo!()
}

impl From<update_connection::Error> for RestError {
    fn from(value: update_connection::Error) -> Self {
        todo!()
    }
}

async fn update_connection(
    State(message_bus): State<MessageBus>,
    account: Account,
    Path(connection_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateConnectionRequest>,
) -> OmniResult<Json<ConnectionResource>> {
    // message_bus
    //     .execute::<update_connection::Handler, _, _>(update_connection::Command {
    //         id: connection_id,
    //         account,
    //         description: req.description,
    //         properties: req.properties,
    //     })
    //     .await?
    //     .map(ConnectionResource::from)
    //     .map(Json::from)
    //     .map_err(RestError::from)
    todo!()
}

async fn delete_connection(
    account: Account,
    State(message_bus): State<MessageBus>,
    Path(connection_id): Path<uuid::Uuid>,
) -> Result<(), RestError> {
    message_bus
        .execute::<delete_connection::Handler, _, _>(delete_connection::Command {
            id: connection_id,
            account,
        })
        .await;
    todo!()
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

async fn create_connection(
    State(message_bus): State<MessageBus>,
    account: Account,
    Json(req): Json<CreateConnectionRequest>,
) -> OmniResult<Json<ConnectionResource>> {
    // message_bus
    //     .execute::<create_connection::CommandHandler, _, _>(create_connection::Command::from((
    //         account, req,
    //     )))
    //     .await?
    //     .map(|c| Json(ConnectionResource::from(c)))
    //     .map_err(RestError::from)
    todo!()
}

impl From<(Account, CreateConnectionRequest)> for create_connection::Command {
    fn from(value: (Account, CreateConnectionRequest)) -> Self {
        create_connection::Command {
            code: value.1.code,
            account: value.0,
            plugin_id: value.1.plugin_id,
            properties: value.1.properties,
            description: value.1.description,
        }
    }
}
