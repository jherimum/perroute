use crate::{
    errors::PerrouteBackofficeApiError,
    rest::api_models::message_type::{
        CreateMessageTypeRequest, MessageTypeResource, UpdateMessageTypeRequest,
    },
};
use axum::{
    extract::{Path, State},
    Json, Router,
};
use perroute_commons::{
    new_id,
    rest::RestError,
    types::{actor::Actor, id::Id},
};
use perroute_cqrs::{
    command_bus::{
        bus::CommandBus,
        commands::{
            CreateMessageTypeCommandBuilder, DeleteMessageTypeCommandBuilder,
            UpdateMessageTypeCommandBuilder,
        },
        handlers::message_type::{
            create_message_type::CreateMessageTypeCommandHandler,
            delete_message_type::DeleteMessageTypeCommandHandler,
            update_message_type::UpdateMessageTypeCommandHandler,
        },
    },
    query_bus::{
        bus::QueryBus,
        handlers::message_type::{
            find_channel_message_type::FindChannelMessageQueryHandler,
            query_channel_message_types::QueryChannelMessageTypesHandler,
        },
        queries::{FindChannelMessageTypeQueryBuilder, QueryChannelMessageTypesQueryBuilder},
    },
};

pub fn routes() -> Router {
    Router::new()
}

async fn retrieve_message_type(
    query_bus: &QueryBus,
    channel_id: &Id,
    message_type_id: &Id,
    actor: &Actor,
) -> Result<Json<MessageTypeResource>, RestError> {
    let query = FindChannelMessageTypeQueryBuilder::default()
        .message_type_id(*message_type_id)
        .channel_id(*channel_id)
        .build()
        .unwrap();

    query_bus
        .execute::<_, FindChannelMessageQueryHandler, _>(actor.clone(), query)
        .await
        .map_err(PerrouteBackofficeApiError::from)?
        .ok_or(RestError::NotFound(format!(
            "Message type {message_type_id} not found"
        )))
        .map(MessageTypeResource::from)
        .map(Json::from)
}

async fn query_message_types(
    State(query_bus): State<QueryBus>,
    Path(channel_id): Path<Id>,
) -> Result<Json<Vec<MessageTypeResource>>, RestError> {
    let query = QueryChannelMessageTypesQueryBuilder::default()
        .channel_id(channel_id)
        .build()
        .unwrap();

    Ok(Json(
        query_bus
            .execute::<_, QueryChannelMessageTypesHandler, _>(Actor::system(), query)
            .await
            .map_err(PerrouteBackofficeApiError::from)?
            .into_iter()
            .map(MessageTypeResource::from)
            .collect::<Vec<_>>(),
    ))
}

async fn find_message_type(
    State(query_bus): State<QueryBus>,
    Path((channel_id, message_type_id)): Path<(Id, Id)>,
) -> Result<Json<MessageTypeResource>, RestError> {
    retrieve_message_type(&query_bus, &channel_id, &message_type_id, &Actor::system()).await
}

async fn create_message_type(
    State(command_bus): State<CommandBus>,
    State(query_bus): State<QueryBus>,
    Path(channel_id): Path<Id>,
    Json(req): Json<CreateMessageTypeRequest>,
) -> Result<Json<MessageTypeResource>, RestError> {
    let command = CreateMessageTypeCommandBuilder::default()
        .message_type_id(new_id!())
        .code(req.code)
        .description(req.description)
        .channel_id(channel_id)
        .build()
        .unwrap();

    command_bus
        .execute::<_, CreateMessageTypeCommandHandler>(Actor::system(), command.clone())
        .await
        .map_err(PerrouteBackofficeApiError::from)?;

    retrieve_message_type(
        &query_bus,
        &channel_id,
        command.message_type_id(),
        &Actor::system(),
    )
    .await
}

async fn update_message_type(
    State(command_bus): State<CommandBus>,
    State(query_bus): State<QueryBus>,
    Path((channel_id, message_type_id)): Path<(Id, Id)>,
    Json(req): Json<UpdateMessageTypeRequest>,
) -> Result<Json<MessageTypeResource>, RestError> {
    let cmd = UpdateMessageTypeCommandBuilder::default()
        .message_type_id(message_type_id)
        .description(req.description)
        .enabled(req.enabled)
        .build()
        .unwrap();

    command_bus
        .execute::<_, UpdateMessageTypeCommandHandler>(Actor::system(), cmd)
        .await
        .map_err(PerrouteBackofficeApiError::from)?;

    retrieve_message_type(&query_bus, &channel_id, &message_type_id, &Actor::system()).await
}

async fn delete_message_type(
    State(command_bus): State<CommandBus>,
    Path((channel_id, message_type_id)): Path<(Id, Id)>,
) -> Result<(), RestError> {
    let cmd = DeleteMessageTypeCommandBuilder::default()
        .message_type_id(message_type_id)
        .build()
        .unwrap();

    command_bus
        .execute::<_, DeleteMessageTypeCommandHandler>(Actor::system(), cmd)
        .await
        .map_err(PerrouteBackofficeApiError::from)?;

    Ok(())
}
