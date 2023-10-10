use crate::{bus::Ctx, command::Command, error::CommandBusError};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor, code::Code, command_type::CommandType, id::Id, vars::Vars,
};
use perroute_storage::{
    models::message_type::{MessageType, MessageTypeBuilder, MessageTypeQuery},
    query::FetchableModel,
};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateMessageTypeError {
    #[error("Code {0} already exists")]
    CodeAlreadyExists(Code),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateMessageTypeCommand {
    #[builder(default)]
    id: Id,
    code: Code,
    name: String,
    vars: Vars,
}

#[async_trait::async_trait]
impl Command for CreateMessageTypeCommand {
    type Output = MessageType;

    #[tracing::instrument(name = "create_message_type_handler", skip(self, ctx))]
    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<MessageType, CommandBusError> {
        if MessageType::exists(ctx.pool(), MessageTypeQuery::with_code(self.code.clone())).await? {
            return Err(CreateMessageTypeError::CodeAlreadyExists(self.code().clone()).into());
        }

        let message_type = MessageTypeBuilder::default()
            .id(self.id)
            .code(self.code.clone())
            .name(self.name.clone())
            .vars(self.vars.clone())
            .build()
            .unwrap()
            .save(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to save message type: {e}"))?;
        Ok(message_type)
    }

    fn command_type(&self) -> CommandType {
        CommandType::CreateMessageType
    }

    fn supports(&self, _actor: &Actor) -> bool {
        true
    }
}
