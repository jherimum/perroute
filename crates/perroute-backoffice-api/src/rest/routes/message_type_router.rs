use crate::{
    errors::PerrouteBackofficeApiError,
    rest::{
        api_models::message_type::{
            CreateMessageTypeRequest, MessageTypeResource, UpdateMessageTypeRequest,
        },
        extractors::{
            actor::ActorExtractor, channel::ChannelExtractor, message_type::MessageTypeExtractor,
        },
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
use std::ops::Deref;

pub struct MessageTypeRouter;

impl MessageTypeRouter {
    pub fn routes(buses: Buses) -> Router {
        Router::new()
            .nest(
                "/v1/channels/:channel_id/message_types",
                Router::new()
                    .route("/", get(Self::query_message_types))
                    .route("/", post(Self::create_message_type))
                    .nest(
                        "/:message_type_id",
                        Router::new()
                            .route("/", get(Self::find_message_type))
                            .route("/", put(Self::update_message_type))
                            .route("/", delete(Self::delete_message_type)),
                    ),
            )
            .with_state(buses)
    }

    async fn retrieve_message_type(
        query_bus: &QueryBus,
        message_type_id: Id,
        actor: &Actor,
    ) -> Result<MessageType, RestError> {
        let query = FindMessageTypeQueryBuilder::default()
            .message_type_id(message_type_id)
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
        channel: ChannelExtractor<Path<Id>>,
    ) -> Result<Json<Vec<MessageTypeResource>>, RestError> {
        let query = QueryMessageTypesQueryBuilder::default()
            .channel_id(*channel.id())
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

    #[tracing::instrument]
    async fn find_message_type(
        message_type: MessageTypeExtractor<Path<(Id, Id)>>,
    ) -> Result<Json<MessageTypeResource>, RestError> {
        Ok(Json::from(MessageTypeResource::from(message_type.deref())))
    }

    async fn create_message_type(
        State(command_bus): State<CommandBus>,
        State(query_bus): State<QueryBus>,
        ActorExtractor(actor): ActorExtractor,
        channel: ChannelExtractor<Path<Id>>,
        Json(body): Json<CreateMessageTypeRequest>,
    ) -> Result<Json<MessageTypeResource>, RestError> {
        let command = CreateMessageTypeCommandBuilder::default()
            .message_type_id(new_id!())
            .code(body.code)
            .description(body.description)
            .channel_id(*channel.id())
            .build()
            .unwrap();

        command_bus
            .execute::<_, CreateMessageTypeCommandHandler>(&actor, command.clone())
            .await
            .map_err(PerrouteBackofficeApiError::from)?;

        Self::retrieve_message_type(&query_bus, *command.message_type_id(), &actor)
            .await
            .map(MessageTypeResource::from)
            .map(Json::from)
    }

    async fn update_message_type(
        State(command_bus): State<CommandBus>,
        State(query_bus): State<QueryBus>,
        ActorExtractor(actor): ActorExtractor,
        message_type: MessageTypeExtractor<Path<(Id, Id)>>,
        Json(req): Json<UpdateMessageTypeRequest>,
    ) -> Result<Json<MessageTypeResource>, RestError> {
        let cmd = UpdateMessageTypeCommandBuilder::default()
            .message_type_id(*message_type.id())
            .description(req.description)
            .enabled(req.enabled)
            .build()
            .unwrap();

        command_bus
            .execute::<_, UpdateMessageTypeCommandHandler>(&actor, cmd)
            .await
            .map_err(PerrouteBackofficeApiError::from)?;

        Self::retrieve_message_type(&query_bus, *message_type.id(), &actor)
            .await
            .map(MessageTypeResource::from)
            .map(Json::from)
    }

    async fn delete_message_type(
        State(command_bus): State<CommandBus>,
        ActorExtractor(actor): ActorExtractor,
        message_type_guard: MessageTypeExtractor<Path<(Id, Id)>>,
    ) -> Result<(), RestError> {
        let cmd = DeleteMessageTypeCommandBuilder::default()
            .message_type_id(*message_type_guard.id())
            .build()
            .unwrap();

        command_bus
            .execute::<_, DeleteMessageTypeCommandHandler>(&actor, cmd)
            .await
            .map_err(PerrouteBackofficeApiError::from)?;

        Ok(())
    }
}
