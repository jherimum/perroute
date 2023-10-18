use crate::{
    api::{
        models::message_type::{
            CreateMessageTypeRequest, MessageTypeResource, MessageTypeRestQuery,
            UpdateMessageTypeRequest,
        },
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
        types::SingleIdPath,
    },
    app::AppState,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
};
use actix_web::web::{Data, Path};
use actix_web_validator::Json;
use actix_web_validator::Query;
use perroute_commandbus::command::message_type::{
    create_message_type::{CreateMessageTypeCommand, CreateMessageTypeCommandBuilder},
    delete_message_type::{DeleteMessageTypeCommand, DeleteMessageTypeCommandBuilder},
    update_message_type::{UpdateMessageTypeCommand, UpdateMessageTypeCommandBuilder},
};
use perroute_cqrs::query_bus::handlers::message_type::find_message_type::FindMessageTypeQueryBuilder;
use perroute_cqrs::query_bus::handlers::message_type::find_message_type::FindMessageTypeQueryHandler;
use perroute_cqrs::query_bus::handlers::message_type::query_message_types::QueryMessageTypesHandler;
use perroute_cqrs::query_bus::handlers::message_type::query_message_types::QueryMessageTypesQueryBuilder;
use tap::TapFallible;

type CollectionResult = ApiResult<CollectionResourceModel<MessageTypeResource>>;
type SingleResult = ApiResult<SingleResourceModel<MessageTypeResource>>;

pub struct MessageTypeRouter;

impl MessageTypeRouter {
    pub const MESSAGE_TYPES_RESOURCE_NAME: &str = "message_types";
    pub const MESSAGE_TYPE_RESOURCE_NAME: &str = "message_type";

    #[tracing::instrument(skip(state))]
    pub async fn query(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Query(request_query): Query<MessageTypeRestQuery>,
    ) -> CollectionResult {
        let query = QueryMessageTypesQueryBuilder::default()
            .build()
            .tap_err(|e| tracing::error!("Failed to build QueryMessageTypesQuery: {e}"))
            .map_err(anyhow::Error::new)?;

        let message_types = state
            .query_bus()
            .execute::<_, QueryMessageTypesHandler, _>(&actor, &query)
            .await
            .tap_err(|e| tracing::error!("Failed to query message types: {e}"))?;

        Ok(ApiResponse::ok((message_types, request_query)))
    }

    #[tracing::instrument(skip(state))]
    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateMessageTypeRequest>,
    ) -> SingleResult {
        let command = CreateMessageTypeCommandBuilder::default()
            .code(body.code()?)
            .name(body.name()?)
            .vars(body.vars()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build CreateMessageTypeCommand:{e}"))
            .map_err(anyhow::Error::new)?;

        Ok(state
            .command_bus()
            .execute::<CreateMessageTypeCommand, _>(actor, command)
            .await
            .tap_err(|e| tracing::error!("Failed to create message type: {e}"))
            .map(|message_type| {
                ApiResponse::created(ResourceLink::MessageType(*message_type.id()), message_type)
            })?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn partial_update(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
        Json(body): Json<UpdateMessageTypeRequest>,
    ) -> SingleResult {
        let command = UpdateMessageTypeCommandBuilder::default()
            .id(path.into_inner().try_into()?)
            .name(body.name()?)
            .vars(body.vars()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build UpdateMessageTypeCommand: {e}"))
            .map_err(anyhow::Error::new)?;

        Ok(state
            .command_bus()
            .execute::<UpdateMessageTypeCommand, _>(actor, command)
            .await
            .tap_err(|e| tracing::error!("Failed to update message type: {e}"))
            .map(ApiResponse::ok)?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn delete(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
    ) -> EmptyApiResult {
        let command = DeleteMessageTypeCommandBuilder::default()
            .id(path.into_inner().try_into()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteMessageTypeCommand: {e}"))
            .map_err(anyhow::Error::new)?;

        Ok(state
            .command_bus()
            .execute::<DeleteMessageTypeCommand, _>(actor, command)
            .await
            .tap_err(|e| tracing::error!("Failed to delete message type: {e}"))
            .map(|_| ApiResponse::ok_empty())?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_one(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
    ) -> SingleResult {
        let query = FindMessageTypeQueryBuilder::default()
            .message_type_id(path.into_inner().try_into()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build FindMessageTypeQuery: {e}"))
            .map_err(anyhow::Error::new)?;

        Ok(state
            .query_bus()
            .execute::<_, FindMessageTypeQueryHandler, _>(&actor, &query)
            .await
            .tap_err(|e| tracing::error!("Faled to retrieve message type:{e}"))
            .map(ApiResponse::ok)?)
    }
}
