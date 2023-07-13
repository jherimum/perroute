use std::str::FromStr;

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
use perroute_commons::types::id::Id;
use perroute_cqrs::command_bus::{
    commands::CreateMessageCommandBuilder,
    handlers::message::create_message::CreateMessageCommandHandler,
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
        let channel_id = req.headers().get("channel_id").unwrap().to_str().unwrap();
        let channel_id = Id::from_str(channel_id).unwrap();
        let cmd = CreateMessageCommandBuilder::default()
            .payload(body.payload.into())
            .message_type_code(body.message_type_code)
            .schema_version(body.schema_version)
            .channel_id(channel_id)
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
