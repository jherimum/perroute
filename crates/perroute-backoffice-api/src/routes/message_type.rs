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
use perroute_cqrs::query_bus::handlers::message_type::find_message_type::FindMessageTypeQueryHandler;
use perroute_cqrs::query_bus::handlers::message_type::query_message_types::QueryMessageTypesHandler;
use perroute_cqrs::query_bus::queries::{
    FindMessageTypeQueryBuilder, QueryMessageTypesQueryBuilder,
};
use perroute_storage::models::message_type::MessageType;
use std::convert::identity;

pub const MESSAGE_TYPES_RESOURCE_NAME: &str = "message_types";
pub const MESSAGE_TYPE_RESOURCE_NAME: &str = "message_type";

pub struct MessageTypeRouter;

impl MessageTypeRouter {
    pub async fn retrieve_message_type<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        message_type_id: Id,
        map: impl FnOnce(MessageType) -> R,
    ) -> Result<R, ApiError> {
        let query = FindMessageTypeQueryBuilder::default()
            .message_type_id(message_type_id)
            .build()
            .unwrap();

        query_bus
            .execute::<_, FindMessageTypeQueryHandler, _>(actor, &query)
            .await?
            .ok_or_else(|| ApiError::MessageTypeNotFound(message_type_id))
            .map(map)
    }

    #[tracing::instrument(skip(state))]
    pub async fn query_message_types(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> ApiResult<MessageTypeResource> {
        let query = QueryMessageTypesQueryBuilder::default().build().unwrap();
        let message_types = state
            .query_bus()
            .execute::<_, QueryMessageTypesHandler, _>(&actor, &query)
            .await?;
        Ok(ApiResponse::OkCollection(message_types.into()))
    }

    #[tracing::instrument(skip(state))]
    pub async fn create_message_type(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateMessageTypeRequest>,
    ) -> ApiResult<MessageTypeResource> {
        let cmd = CreateMessageTypeCommandBuilder::default()
            .message_type_id(new_id!())
            .channel_id(*body.channel_id())
            .code(body.code().clone())
            .description(body.description().to_owned())
            .build()
            .unwrap();

        Ok(state
            .command_bus()
            .execute::<_, CreateMessageTypeCommandHandler, _>(&actor, &cmd)
            .await
            .map(|message_type| {
                ApiResponse::Created(
                    ResourceLink::MessageType(*message_type.id()),
                    message_type.into(),
                )
            })?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn update_message_type(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        message_types_path: Path<Id>,
        Json(body): Json<UpdateMessageTypeRequest>,
    ) -> ApiResult<MessageTypeResource> {
        let message_type = Self::retrieve_message_type(
            state.query_bus(),
            &actor,
            message_types_path.into_inner(),
            identity,
        )
        .await?;

        let cmd = UpdateMessageTypeCommandBuilder::default()
            .message_type_id(*message_type.id())
            .description(body.description().to_owned())
            .enabled(*body.enabled())
            .build()
            .unwrap();

        Ok(state
            .command_bus()
            .execute::<_, UpdateMessageTypeCommandHandler, _>(&actor, &cmd)
            .await
            .map(|message_type| ApiResponse::OkSingle(message_type.into()))?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn delete_message_type(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        message_types_path: Path<Id>,
    ) -> ApiResult<EmptyResource> {
        let message_type = Self::retrieve_message_type(
            state.query_bus(),
            &actor,
            message_types_path.into_inner(),
            identity,
        )
        .await?;

        let cmd = DeleteMessageTypeCommandBuilder::default()
            .message_type_id(*message_type.id())
            .build()
            .unwrap();

        Ok(state
            .command_bus()
            .execute::<_, DeleteMessageTypeCommandHandler, _>(&actor, &cmd)
            .await
            .map(|_| ApiResponse::OkEmpty(EmptyResource))?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_message_type(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> ApiResult<MessageTypeResource> {
        Self::retrieve_message_type(
            state.query_bus(),
            &actor,
            path.into_inner(),
            |message_type| ApiResponse::OkSingle(message_type.into()),
        )
        .await
    }
}
