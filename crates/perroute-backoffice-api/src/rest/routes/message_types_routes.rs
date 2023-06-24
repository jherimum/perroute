use crate::{
    errors::PerrouteBackofficeApiError,
    rest::{
        api_models::message_type::{
            CreateMessageTypeRequest, MessageTypeResource, UpdateMessageTypeRequest,
        },
        extractors::{actor::ActorExtractor, channel::ChannelResourceGuard},
        Buses,
    },
};
use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
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
            find_message_type::FindMessageTypeQueryHandler,
            query_message_types::QueryMessageTypesHandler,
        },
        queries::{FindMessageTypeQueryBuilder, QueryMessageTypesQueryBuilder},
    },
};
use perroute_storage::models::message_type::MessageType;

pub fn routes(buses: Buses) -> Router {
    Router::new()
        .nest(
            "/v1/channels/:channel_id/message_types",
            Router::new()
                .route("/", get(query_message_types))
                .route("/", post(create_message_type))
                .nest(
                    "/:id",
                    Router::new()
                        .route("/", get(find_message_type))
                        .route("/", put(update_message_type))
                        .route("/", delete(delete_message_type)),
                ),
        )
        .with_state(buses)
}

async fn retrieve_message_type(
    query_bus: &QueryBus,
    channel_id: &Id,
    message_type_id: &Id,
    actor: &Actor,
) -> Result<MessageType, RestError> {
    let query = FindMessageTypeQueryBuilder::default()
        .message_type_id(*message_type_id)
        .channel_id(*channel_id)
        .build()
        .unwrap();

    query_bus
        .execute::<_, FindMessageTypeQueryHandler, _>(actor, query)
        .await
        .map_err(PerrouteBackofficeApiError::from)?
        .ok_or(RestError::NotFound(format!(
            "Message type {message_type_id} not found"
        )))
}

#[tracing::instrument(skip(query_bus))]
async fn query_message_types(
    State(query_bus): State<QueryBus>,
    ActorExtractor(actor): ActorExtractor,
    Path(channel_id): Path<Id>,
    _: ChannelResourceGuard<Path<Id>>,
) -> Result<Json<Vec<MessageTypeResource>>, RestError> {
    let query = QueryMessageTypesQueryBuilder::default()
        .channel_id(channel_id)
        .build()
        .unwrap();

    Ok(Json(
        query_bus
            .execute::<_, QueryMessageTypesHandler, _>(&actor, query)
            .await
            .map_err(PerrouteBackofficeApiError::from)?
            .into_iter()
            .map(MessageTypeResource::from)
            .collect::<Vec<_>>(),
    ))
}

#[tracing::instrument(skip(query_bus))]
async fn find_message_type(
    State(query_bus): State<QueryBus>,
    Path((channel_id, message_type_id)): Path<(Id, Id)>,
) -> Result<Json<MessageTypeResource>, RestError> {
    retrieve_message_type(&query_bus, &channel_id, &message_type_id, &Actor::system())
        .await
        .map(MessageTypeResource::from)
        .map(Json::from)
}

#[tracing::instrument(skip(query_bus, command_bus))]
async fn create_message_type(
    State(command_bus): State<CommandBus>,
    State(query_bus): State<QueryBus>,
    ActorExtractor(actor): ActorExtractor,
    Path(channel_id): Path<Id>,
    _: ChannelResourceGuard<Path<Id>>,
    Json(body): Json<CreateMessageTypeRequest>,
) -> Result<Json<MessageTypeResource>, RestError> {
    let command = CreateMessageTypeCommandBuilder::default()
        .message_type_id(new_id!())
        .code(body.code)
        .description(body.description)
        .channel_id(channel_id)
        .build()
        .unwrap();

    command_bus
        .execute::<_, CreateMessageTypeCommandHandler>(&actor, command.clone())
        .await
        .map_err(PerrouteBackofficeApiError::from)?;

    retrieve_message_type(
        &query_bus,
        &channel_id,
        command.message_type_id(),
        &actor,
    )
    .await
    .map(MessageTypeResource::from)
    .map(Json::from)
}

#[tracing::instrument(skip(query_bus, command_bus))]
async fn update_message_type(
    State(command_bus): State<CommandBus>,
    State(query_bus): State<QueryBus>,
    ActorExtractor(actor): ActorExtractor,
    Path((channel_id, message_type_id)): Path<(Id, Id)>,
    Json(req): Json<UpdateMessageTypeRequest>,
) -> Result<Json<MessageTypeResource>, RestError> {
    retrieve_message_type(&query_bus, &channel_id, &message_type_id, &Actor::system()).await?;

    let cmd = UpdateMessageTypeCommandBuilder::default()
        .message_type_id(message_type_id)
        .description(req.description)
        .enabled(req.enabled)
        .build()
        .unwrap();

    command_bus
        .execute::<_, UpdateMessageTypeCommandHandler>(&actor, cmd)
        .await
        .map_err(PerrouteBackofficeApiError::from)?;

    retrieve_message_type(&query_bus, &channel_id, &message_type_id, &Actor::system())
        .await
        .map(MessageTypeResource::from)
        .map(Json::from)
}

#[tracing::instrument(skip(query_bus, command_bus))]
async fn delete_message_type(
    State(query_bus): State<QueryBus>,
    State(command_bus): State<CommandBus>,
    ActorExtractor(actor): ActorExtractor,
    Path((channel_id, message_type_id)): Path<(Id, Id)>,
) -> Result<(), RestError> {
    retrieve_message_type(&query_bus, &channel_id, &message_type_id, &Actor::system()).await?;

    let cmd = DeleteMessageTypeCommandBuilder::default()
        .message_type_id(message_type_id)
        .build()
        .unwrap();

    command_bus
        .execute::<_, DeleteMessageTypeCommandHandler>(&actor, cmd)
        .await
        .map_err(PerrouteBackofficeApiError::from)?;

    Ok(())
}
