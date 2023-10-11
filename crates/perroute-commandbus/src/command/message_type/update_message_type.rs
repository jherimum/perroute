use crate::{bus::Ctx, command::Command, error::CommandBusError};
use perroute_commons::types::{actor::Actor, command_type::CommandType, id::Id, vars::Vars};
use perroute_storage::{
    models::message_type::{MessageType, MessageTypeQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum UpdateMessageTypeError {
    #[error("Message type with id {0} not found")]
    MessageTypeNotFound(Id),
}

#[derive(Debug, derive_builder::Builder)]
pub struct UpdateMessageTypeCommand {
    id: Id,
    name: Option<String>,
    vars: Option<Vars>,
}

#[async_trait::async_trait]
impl Command for UpdateMessageTypeCommand {
    type Output = MessageType;

    #[tracing::instrument(name = "update_message_type_handler", skip(self, ctx))]
    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        let mut message_type = MessageType::find(ctx.pool(), MessageTypeQuery::with_id(self.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve message type {}:{e}", self.id))?
            .ok_or(UpdateMessageTypeError::MessageTypeNotFound(self.id))?;

        if self.name.is_none() & self.vars.is_none() {
            return Ok(message_type);
        }

        if let Some(name) = self.name.clone() {
            message_type = message_type.set_name(name);
        }

        if let Some(vars) = self.vars.clone() {
            message_type = message_type.set_vars(vars);
        }

        Ok(message_type
            .update(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to update message type {}: {e}", self.id))?)
    }

    fn supports(&self, _actor: &Actor) -> bool {
        true
    }

    fn command_type(&self) -> CommandType {
        CommandType::UpdateMessageType
    }
}
