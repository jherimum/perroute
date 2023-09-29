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
    W,
};
use actix_web::web::{Data, Path};
use actix_web_validator::Json;
use actix_web_validator::Query;
use perroute_cqrs::{
    command_bus::handlers::message_type::create_message_type::CreateMessageTypeCommandHandler,
    query_bus::handlers::message_type::{
        find_message_type::FindMessageTypeQuery, query_message_types::QueryMessageTypesQueryBuilder,
    },
};
use perroute_cqrs::{
    command_bus::handlers::message_type::update_message_type::UpdateMessageTypeCommandBuilder,
    query_bus::handlers::message_type::query_message_types::QueryMessageTypesHandler,
};
use perroute_cqrs::{
    command_bus::handlers::message_type::update_message_type::UpdateMessageTypeCommandHandler,
    query_bus::handlers::message_type::find_message_type::FindMessageTypeQueryBuilder,
};
use perroute_cqrs::{
    command_bus::handlers::message_type::{
        create_message_type::CreateMessageTypeCommand,
        delete_message_type::{DeleteMessageTypeCommand, DeleteMessageTypeCommandHandler},
        update_message_type::UpdateMessageTypeCommand,
    },
    query_bus::handlers::message_type::query_message_types::QueryMessageTypesQuery,
};
use perroute_cqrs::{
    command_bus::handlers::message_type::{
        create_message_type::CreateMessageTypeCommandBuilder,
        delete_message_type::DeleteMessageTypeCommandBuilder,
    },
    query_bus::handlers::message_type::find_message_type::FindMessageTypeQueryHandler,
};
use tap::TapFallible;

impl TryInto<CreateMessageTypeCommand> for CreateMessageTypeRequest {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<CreateMessageTypeCommand, Self::Error> {
        Ok(CreateMessageTypeCommandBuilder::default()
            .code(self.code()?)
            .name(self.name()?)
            .vars(self.vars()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build CreateMessageTypeCommand:{e}"))?)
    }
}

impl TryInto<UpdateMessageTypeCommand> for W<(Path<SingleIdPath>, UpdateMessageTypeRequest)> {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<UpdateMessageTypeCommand, Self::Error> {
        let w = self.into_inner();
        Ok(UpdateMessageTypeCommandBuilder::default()
            .id(w.0.into_inner().try_into()?)
            .name(w.1.name()?)
            .enabled(w.1.enabled()?)
            .vars(w.1.vars()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build UpdateMessageTypeCommand: {e}"))?)
    }
}

impl TryInto<DeleteMessageTypeCommand> for W<Path<SingleIdPath>> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<DeleteMessageTypeCommand, Self::Error> {
        Ok(DeleteMessageTypeCommandBuilder::default()
            .id(self.into_inner().into_inner().try_into()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteMessageTypeCommand: {e}"))?)
    }
}

impl TryInto<QueryMessageTypesQuery> for MessageTypeRestQuery {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<QueryMessageTypesQuery, Self::Error> {
        Ok(QueryMessageTypesQueryBuilder::default()
            .build()
            .tap_err(|e| tracing::error!("Failed to build QueryMessageTypesQuery: {e}"))?)
    }
}

impl TryInto<FindMessageTypeQuery> for W<Path<SingleIdPath>> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<FindMessageTypeQuery, Self::Error> {
        Ok(FindMessageTypeQueryBuilder::default()
            .message_type_id(self.into_inner().into_inner().try_into()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build FindMessageTypeQuery: {e}"))?)
    }
}

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
        Query(query): Query<MessageTypeRestQuery>,
    ) -> CollectionResult {
        let message_types = state
            .query_bus()
            .execute::<_, QueryMessageTypesHandler, _>(&actor, &query.clone().try_into()?)
            .await
            .tap_err(|e| tracing::error!("Failed to query message types: {e}"))?;

        Ok(ApiResponse::ok((message_types, query)))
    }

    #[tracing::instrument(skip(state))]
    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateMessageTypeRequest>,
    ) -> SingleResult {
        Ok(state
            .command_bus()
            .execute::<_, CreateMessageTypeCommandHandler, _>(&actor, &body.try_into()?)
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
        Ok(state
            .command_bus()
            .execute::<_, UpdateMessageTypeCommandHandler, _>(&actor, &W((path, body)).try_into()?)
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
        Ok(state
            .command_bus()
            .execute::<_, DeleteMessageTypeCommandHandler, _>(&actor, &W(path).try_into()?)
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
        Ok(state
            .query_bus()
            .execute::<_, FindMessageTypeQueryHandler, _>(&actor, &W(path).try_into()?)
            .await
            .tap_err(|e| tracing::error!("Faled to retrieve message type:{e}"))
            .map(ApiResponse::ok)?)
    }
}
