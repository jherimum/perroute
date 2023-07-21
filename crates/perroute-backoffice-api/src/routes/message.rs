use crate::{
    api::{
        models::message::{CreateMessageRequest, MessageResource},
        response::{ApiResponse, ApiResult, SingleResourceModel},
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
};
use actix_web::web::{Data, Json};
use perroute_commons::types::actor::Actor;
use perroute_cqrs::{
    command_bus::{
        commands::CreateMessageCommandBuilder,
        handlers::message::create_message::CreateMessageCommandHandler,
    },
    query_bus::{
        handlers::schema::find_schema::FindSchemaQueryHandler, queries::FindSchemaQueryBuilder,
    },
};
use perroute_storage::models::{message::Message, schema::Schema};

pub type SingleResult = ApiResult<SingleResourceModel<MessageResource>>;

pub struct MessageRouter;

impl MessageRouter {
    pub const MESSAGES_RESOURCE_NAME: &str = "messages";

    #[tracing::instrument(skip(state))]
    pub async fn create_message(
        ActorExtractor(actor): ActorExtractor,
        state: Data<AppState>,
        Json(body): Json<CreateMessageRequest>,
    ) -> SingleResult {
        let schema = retrieve_schema(&state, &actor, &body).await?.unwrap();
        let message = create_message(&state, &actor, &schema, body).await?;
        Ok(ApiResponse::ok(message))
    }
}

async fn create_message(
    state: &AppState,
    actor: &Actor,
    schema: &Schema,
    body: CreateMessageRequest,
) -> Result<Message, ApiError> {
    let cmd = CreateMessageCommandBuilder::default()
        .payload(body.payload.into())
        .schema_id(*schema.id())
        .recipient(body.recipient)
        .scheduled_to(body.scheduled_to)
        .include_dispatcher_types(body.include_dispatcher_types)
        .exclude_dispatcher_types(body.exclude_dispatcher_types)
        .build()
        .unwrap();
    state
        .command_bus()
        .execute::<_, CreateMessageCommandHandler, _>(&actor, &cmd)
        .await
        .map_err(ApiError::from)
}

async fn retrieve_schema(
    state: &AppState,
    actor: &Actor,
    body: &CreateMessageRequest,
) -> Result<Option<Schema>, ApiError> {
    let schema_query = FindSchemaQueryBuilder::default()
        .channel_code(Some(body.channel_code.clone()))
        .message_type_code(Some(body.message_type_code.clone()))
        .version(Some(body.schema_version))
        .build()
        .unwrap();

    state
        .query_bus()
        .execute::<_, FindSchemaQueryHandler, _>(&actor, &schema_query)
        .await
        .map_err(ApiError::from)
}
