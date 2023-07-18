use crate::{
    api::{
        models::message::{CreateMessageRequest, MessageResource},
        response::{ApiResponse, ApiResult, ResourceModel},
    },
    app::AppState,
    extractors::actor::ActorExtractor,
};
use actix_web::{
    web::{Data, Json},
    HttpRequest,
};
use perroute_cqrs::{
    command_bus::{
        commands::CreateMessageCommandBuilder,
        handlers::message::create_message::CreateMessageCommandHandler,
    },
    query_bus::{
        handlers::schema::find_schema::FindSchemaQueryHandler, queries::FindSchemaQueryBuilder,
    },
};

pub type SingleResult = ApiResult<ResourceModel<MessageResource>>;

pub struct MessageRouter {}

impl MessageRouter {
    pub const MESSAGES_RESOURCE_NAME: &str = "messages";

    #[tracing::instrument(skip(state))]
    pub async fn create_message(
        ActorExtractor(actor): ActorExtractor,
        state: Data<AppState>,
        Json(body): Json<CreateMessageRequest>,
        req: HttpRequest,
    ) -> SingleResult {
        let schema_query = FindSchemaQueryBuilder::default()
            .channel_code(Some(body.channel_code.clone()))
            .message_type_code(Some(body.message_type_code))
            .version(Some(body.schema_version))
            .message_type_id(None)
            .schema_id(None)
            .build()
            .unwrap();

        let schema = state
            .query_bus()
            .execute::<_, FindSchemaQueryHandler, _>(&actor, &schema_query)
            .await
            .unwrap()
            .unwrap();

        let cmd = CreateMessageCommandBuilder::default()
            .payload(body.payload.into())
            .schema_id(*schema.id())
            .build()
            .unwrap();
        let message = state
            .command_bus()
            .execute::<_, CreateMessageCommandHandler, _>(&actor, &cmd)
            .await
            .unwrap();
        Ok(ApiResponse::ok(message))
    }
}
