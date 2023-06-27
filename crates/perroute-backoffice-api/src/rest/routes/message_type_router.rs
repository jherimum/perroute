use crate::{
    errors::PerrouteBackofficeApiError,
    rest::{
        api_models::message_type::{
            CreateMessageTypeRequest, MessageTypeResource, UpdateMessageTypeRequest,
        },
        extractors::{
            actor::ActorExtractor,
            resource_path::{ChannelPath, MessageTypePath, ResourcePath},
        },
        Buses,
    },
};
use axum::{
    extract::State,
    routing::{delete, get, post, put},
    Json, Router,
};
use perroute_commons::{new_id, rest::RestError};
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
        bus::QueryBus, handlers::message_type::query_message_types::QueryMessageTypesHandler,
        queries::QueryMessageTypesQueryBuilder,
    },
};

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

    #[tracing::instrument(skip(query_bus))]
    async fn query_message_types(
        State(query_bus): State<QueryBus>,
        ActorExtractor(actor): ActorExtractor,
        channel_path: ChannelPath,
    ) -> Result<Json<Vec<MessageTypeResource>>, RestError> {
        let channel = channel_path
            .resource(&query_bus, &actor)
            .await?
            .ok_or(RestError::NotFound("".to_owned()))?;

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
        message_type_path: MessageTypePath,
        State(query_bus): State<QueryBus>,
        ActorExtractor(actor): ActorExtractor,
    ) -> Result<Json<MessageTypeResource>, RestError> {
        Ok(Json(MessageTypeResource::from(
            message_type_path
                .resource(&query_bus, &actor)
                .await?
                .ok_or(RestError::NotFound("".to_owned()))?,
        )))
    }

    async fn create_message_type(
        State(command_bus): State<CommandBus>,
        State(query_bus): State<QueryBus>,
        ActorExtractor(actor): ActorExtractor,
        channel_path: ChannelPath,
        Json(body): Json<CreateMessageTypeRequest>,
    ) -> Result<Json<MessageTypeResource>, RestError> {
        let channel = channel_path
            .resource(&query_bus, &actor)
            .await?
            .ok_or(RestError::NotFound("".to_owned()))?;
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

        Ok(Json(MessageTypeResource::from(
            MessageTypePath::from((*channel.id(), *command.message_type_id()))
                .resource(&query_bus, &actor)
                .await?
                .ok_or(RestError::NotFound("".to_owned()))?,
        )))
    }

    async fn update_message_type(
        State(command_bus): State<CommandBus>,
        State(query_bus): State<QueryBus>,
        ActorExtractor(actor): ActorExtractor,
        message_type_path: MessageTypePath,
        Json(req): Json<UpdateMessageTypeRequest>,
    ) -> Result<Json<MessageTypeResource>, RestError> {
        let message_type = message_type_path
            .resource(&query_bus, &actor)
            .await?
            .ok_or(RestError::NotFound("".to_owned()))?;
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

        Ok(Json(MessageTypeResource::from(
            message_type_path
                .resource(&query_bus, &actor)
                .await?
                .ok_or(RestError::NotFound("".to_owned()))?,
        )))
    }

    async fn delete_message_type(
        State(command_bus): State<CommandBus>,
        State(query_bus): State<QueryBus>,
        ActorExtractor(actor): ActorExtractor,
        message_type_path: MessageTypePath,
    ) -> Result<(), RestError> {
        let message_type = message_type_path
            .resource(&query_bus, &actor)
            .await?
            .ok_or(RestError::NotFound("".to_owned()))?;
        let cmd = DeleteMessageTypeCommandBuilder::default()
            .message_type_id(*message_type.id())
            .build()
            .unwrap();

        command_bus
            .execute::<_, DeleteMessageTypeCommandHandler>(&actor, cmd)
            .await
            .map_err(PerrouteBackofficeApiError::from)?;

        Ok(())
    }
}
