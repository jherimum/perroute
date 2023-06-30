use crate::api::response::{ApiResponse, EmptyResource};
use crate::api::ResourceLink;
use crate::{
    api::{
        models::message_type::{
            CreateMessageTypeRequest, MessageTypeResource, UpdateMessageTypeRequest,
        },
        response::ApiResult,
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
};
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Path;
use perroute_commons::new_id;
use perroute_commons::types::actor::Actor;
use perroute_commons::types::id::Id;
use perroute_cqrs::command_bus::commands::{
    DeleteMessageTypeCommandBuilder, UpdateMessageTypeCommandBuilder,
};
use perroute_cqrs::command_bus::handlers::message_type::delete_message_type::DeleteMessageTypeCommandHandler;
use perroute_cqrs::command_bus::handlers::message_type::update_message_type::UpdateMessageTypeCommandHandler;
use perroute_cqrs::command_bus::{
    commands::CreateMessageTypeCommandBuilder,
    handlers::message_type::create_message_type::CreateMessageTypeCommandHandler,
};
use perroute_cqrs::query_bus::bus::QueryBus;
use perroute_cqrs::query_bus::handlers::message_type::find_channel_message_type::FindChannelMessageTypeQueryHandler;
use perroute_cqrs::query_bus::handlers::message_type::find_message_type::FindMessageTypeQueryHandler;
use perroute_cqrs::query_bus::handlers::message_type::query_message_types::QueryMessageTypesHandler;
use perroute_cqrs::query_bus::queries::{
    FindChannelMessageTypeQueryBuilder, FindMessageTypeQueryBuilder, QueryMessageTypesQueryBuilder,
};
use perroute_storage::models::message_type::MessageType;

use super::channel::ChannelRouter;

pub const MESSAGE_TYPES_RESOURCE_NAME: &str = "message_types";
pub const MESSAGE_TYPE_RESOURCE_NAME: &str = "message_type";

pub struct MessageTypeRouter;

impl MessageTypeRouter {
    #[tracing::instrument]
    pub async fn query(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> ApiResult<MessageTypeResource> {
        println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

        let channel_id = path.into_inner();
        let channel =
            ChannelRouter::retrieve_channel(state.query_bus(), &actor, &channel_id, || {
                ApiError::ChannelNotFound(channel_id)
            })
            .await?;
        let query = QueryMessageTypesQueryBuilder::default()
            .channel_id(*channel.id())
            .build()
            .unwrap();

        let message_types = state
            .query_bus()
            .execute::<_, QueryMessageTypesHandler, _>(&actor, &query)
            .await?;

        Ok(ApiResponse::OkCollection((channel, message_types).into()))
    }

    #[tracing::instrument]
    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
        Json(body): Json<CreateMessageTypeRequest>,
    ) -> ApiResult<MessageTypeResource> {
        let channel_id = path.into_inner();
        let channel =
            ChannelRouter::retrieve_channel(state.query_bus(), &actor, &channel_id, || {
                ApiError::ChannelNotFound(channel_id)
            })
            .await?;

        let cmd = CreateMessageTypeCommandBuilder::default()
            .message_type_id(new_id!())
            .channel_id(*channel.id())
            .code(body.code().clone())
            .description(body.description().to_owned())
            .build()
            .unwrap();
        state
            .command_bus()
            .execute::<_, CreateMessageTypeCommandHandler>(&actor, &cmd)
            .await?;

        let query = FindMessageTypeQueryBuilder::default()
            .message_type_id(*cmd.message_type_id())
            .build()
            .unwrap();

        let message_type = state
            .query_bus()
            .execute::<_, FindMessageTypeQueryHandler, _>(&actor, &query)
            .await?
            .unwrap();

        Ok(ApiResponse::Created(
            ResourceLink::MessageType(channel_id, *message_type.id()),
            message_type.into(),
        ))
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn update(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id)>,
        Json(body): Json<UpdateMessageTypeRequest>,
    ) -> ApiResult<MessageTypeResource> {
        let path = path.into_inner();
        let message_type =
            Self::retrieve_message_type(state.query_bus(), &actor, &path.0, &path.1, || {
                ApiError::MessageTypeNotFound(path.1)
            })
            .await?;

        let cmd = UpdateMessageTypeCommandBuilder::default()
            .message_type_id(*message_type.id())
            .description(body.description().to_owned())
            .enabled(*body.enabled())
            .build()
            .unwrap();
        state
            .command_bus()
            .execute::<_, UpdateMessageTypeCommandHandler>(&actor, &cmd)
            .await?;

        let message_type =
            Self::retrieve_message_type(state.query_bus(), &actor, &path.0, &path.1, || {
                ApiError::MessageTypeNotFound(path.1)
            })
            .await?;

        Ok(ApiResponse::OkSingle(message_type.into()))
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn delete(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id)>,
    ) -> ApiResult<EmptyResource> {
        let message_type =
            Self::retrieve_message_type(state.query_bus(), &actor, &path.0, &path.1, || {
                ApiError::ChannelNotFound(path.1)
            })
            .await?;

        let cmd = DeleteMessageTypeCommandBuilder::default()
            .message_type_id(*message_type.id())
            .build()
            .unwrap();

        state
            .command_bus()
            .execute::<_, DeleteMessageTypeCommandHandler>(&actor, &cmd)
            .await?;

        Ok(ApiResponse::OkEmpty(EmptyResource))
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn find(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id)>,
    ) -> ApiResult<MessageTypeResource> {
        let message_type =
            Self::retrieve_message_type(state.query_bus(), &actor, &path.0, &path.1, || {
                ApiError::MessageTypeNotFound(path.0)
            })
            .await?;
        Ok(ApiResponse::OkSingle(message_type.into()))
    }

    pub async fn retrieve_message_type(
        query_bus: &QueryBus,
        actor: &Actor,
        channel_id: &Id,
        message_type_id: &Id,
        not_found: impl FnOnce() -> ApiError,
    ) -> Result<MessageType, ApiError> {
        let query = FindChannelMessageTypeQueryBuilder::default()
            .channel_id(*channel_id)
            .message_type_id(*message_type_id)
            .build()
            .unwrap();

        query_bus
            .execute::<_, FindChannelMessageTypeQueryHandler, _>(&actor, &query)
            .await?
            .ok_or_else(not_found)
    }
}
