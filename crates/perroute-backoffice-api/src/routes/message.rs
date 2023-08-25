use crate::{
    api::{
        models::message::{rest_to_delivery, CreateMessageRequest, MessageResource},
        response::{ApiResponse, ApiResult, SingleResourceModel},
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
};
use actix_web::{web::Data, HttpResponse};
use actix_web_validator::Json;
use anyhow::Context;
use perroute_commons::types::{actor::Actor, version::Version};
use perroute_connectors::types::delivery::Delivery;
use perroute_cqrs::command_bus::handlers::message::create_message::{
    CreateMessageCommandBuilder, CreateMessageCommandHandler,
};
use perroute_storage::models::message::Message;

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
        let m = create_message(&state, &actor, body).await?;
        Ok(ApiResponse::ok(m))
    }
}

async fn create_message(
    state: &AppState,
    actor: &Actor,
    body: CreateMessageRequest,
) -> Result<Message, ApiError> {
    let cmd = CreateMessageCommandBuilder::default()
        .payload(body.payload.into())
        .business_unit_code(body.bu_code.unwrap().try_into().context("context")?)
        .message_type_code(
            body.message_type_code
                .unwrap()
                .try_into()
                .context("context")?,
        )
        .schema_version(body.schema_version.unwrap().into())
        .deliveries(rest_to_delivery(body.deliveries)?)
        .build()
        .unwrap();
    state
        .command_bus()
        .execute::<_, CreateMessageCommandHandler, _>(actor, &cmd)
        .await
        .map_err(ApiError::from)
}
