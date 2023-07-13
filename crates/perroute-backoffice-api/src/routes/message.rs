use crate::{
    api::{
        models::message::{CreateMessageRequest, MessageResource},
        response::{ApiResponse, ApiResult, ResourceModel},
    },
    app::AppState,
    extractors::actor::ActorExtractor,
};
use actix_web::web::{Data, Json};
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
    ) -> SingleResult {
        let cmd = CreateMessageCommandBuilder::default().build().unwrap();
        let message = state
            .command_bus()
            .execute::<_, CreateMessageCommandHandler, _>(&actor, &cmd)
            .await
            .unwrap();
        Ok(ApiResponse::ok(message))
    }
}
