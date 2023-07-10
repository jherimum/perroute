use super::channel::ChannelRouter;
use crate::{
    api::{
        models::message_type::{
            CreateMessageTypeRequest, MessageTypeResource, UpdateMessageTypeRequest,
        },
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
};
use actix_web::web::{Data, Json, Path};
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_cqrs::query_bus::handlers::message_type::find_message_type::FindMessageTypeQueryHandler;
use perroute_cqrs::query_bus::handlers::message_type::query_message_types::QueryMessageTypesHandler;
use perroute_cqrs::{
    command_bus::handlers::message_type::create_message_type::CreateMessageTypeCommandHandler,
    query_bus::queries::QueryMessageTypesQueryBuilder,
};
use perroute_cqrs::{
    command_bus::{
        commands::UpdateMessageTypeCommandBuilder,
        handlers::message_type::update_message_type::UpdateMessageTypeCommandHandler,
    },
    query_bus::queries::FindMessageTypeQueryBuilder,
};
use perroute_cqrs::{
    command_bus::{
        commands::{CreateMessageTypeCommandBuilder, DeleteMessageTypeCommandBuilder},
        handlers::message_type::delete_message_type::DeleteMessageTypeCommandHandler,
    },
    query_bus::bus::QueryBus,
};
use perroute_storage::models::message_type::MessageType;
use std::convert::identity;
use tap::TapFallible;

pub const MESSAGE_TYPES_RESOURCE_NAME: &str = "message_types";
pub const MESSAGE_TYPE_RESOURCE_NAME: &str = "message_type";

type CollectionResult = ApiResult<CollectionResourceModel<MessageTypeResource>>;
type SingleResult = ApiResult<SingleResourceModel<MessageTypeResource>>;

pub struct MessageTypeRouter;

impl MessageTypeRouter {
    #[tracing::instrument(skip(state))]
    pub async fn query_message_types(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> CollectionResult {
        let channel =
            ChannelRouter::retrieve_channel(state.query_bus(), &actor, *path.as_ref(), identity)
                .await?;

        let query = QueryMessageTypesQueryBuilder::default()
            .channel_id(*channel.id())
            .build()
            .tap_err(|e| tracing::error!("Failed to build QueryMessageTypesQuery: {e}"))?;

        let message_types = state
            .query_bus()
            .execute::<_, QueryMessageTypesHandler, _>(&actor, &query)
            .await
            .tap_err(|e| tracing::error!("Failed to query message types: {e}"))?;

        Ok(ApiResponse::ok((channel, message_types)))
    }

    #[tracing::instrument(skip(state))]
    pub async fn create_message_type(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateMessageTypeRequest>,
        path: Path<Id>,
    ) -> SingleResult {
        let channel =
            ChannelRouter::retrieve_channel(state.query_bus(), &actor, *path.as_ref(), identity)
                .await?;

        let cmd = CreateMessageTypeCommandBuilder::default()
            .channel_id(*channel.id())
            .code(body.code().clone())
            .description(body.description().to_owned())
            .build()
            .tap_err(|e| tracing::error!("Failed to build CreateMessageTypeCommand:{e}"))?;

        Ok(state
            .command_bus()
            .execute::<_, CreateMessageTypeCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to create message type: {e}"))
            .map(|message_type| {
                ApiResponse::created(
                    ResourceLink::MessageType(*message_type.channel_id(), *message_type.id()),
                    message_type,
                )
            })?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn update_message_type(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id)>,
        Json(body): Json<UpdateMessageTypeRequest>,
    ) -> SingleResult {
        let message_type =
            Self::retrieve_message_type(state.query_bus(), &actor, *path.as_ref(), identity)
                .await?;

        let cmd = UpdateMessageTypeCommandBuilder::default()
            .message_type_id(*message_type.id())
            .description(body.description().to_owned())
            .enabled(*body.enabled())
            .build()
            .tap_err(|e| tracing::error!("Failed to build UpdateMessageTypeCommand: {e}"))?;

        Ok(state
            .command_bus()
            .execute::<_, UpdateMessageTypeCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to update message type: {e}"))
            .map(ApiResponse::ok)?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn delete_message_type(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id)>,
    ) -> EmptyApiResult {
        let message_type =
            Self::retrieve_message_type(state.query_bus(), &actor, *path.as_ref(), identity)
                .await?;

        let cmd = DeleteMessageTypeCommandBuilder::default()
            .message_type_id(*message_type.id())
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteMessageTypeCommand: {e}"))?;

        Ok(state
            .command_bus()
            .execute::<_, DeleteMessageTypeCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to delete message type: {e}"))
            .map(|_| ApiResponse::ok_empty())?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_message_type(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id)>,
    ) -> SingleResult {
        Self::retrieve_message_type(state.query_bus(), &actor, *path.as_ref(), ApiResponse::ok)
            .await
    }

    pub async fn retrieve_message_type<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        path: (Id, Id),
        map: impl FnOnce(MessageType) -> R,
    ) -> Result<R, ApiError> {
        let query = FindMessageTypeQueryBuilder::default()
            .channel_id(Some(path.0))
            .message_type_id(path.1)
            .build()
            .tap_err(|e| tracing::error!("Failed to build FindMessageTypeQuery: {e}"))?;

        query_bus
            .execute::<_, FindMessageTypeQueryHandler, _>(actor, &query)
            .await
            .tap_err(|e| tracing::error!("Faled to retrieve message type:{e}"))?
            .ok_or_else(|| ApiError::MessageTypeNotFound(path.0))
            .map(map)
    }
}
