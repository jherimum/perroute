use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use perroute_commons::types::{actor::Actor, id::Id, vars::Vars};
use perroute_storage::{
    models::message_type::{MessageType, MessageTypeQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Message type with id {0} not found")]
    MessageTypeNotFound(Id),
}

command!(
    UpdateMessageTypeCommand,
    CommandType::UpdateMessageType,
    id: Id,
    name: Option<String>,
    enabled: Option<bool>,
    vars: Option<Vars>

);
into_event!(UpdateMessageTypeCommand);

#[derive(Debug)]
pub struct UpdateMessageTypeCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for UpdateMessageTypeCommandHandler {
    type Command = UpdateMessageTypeCommand;
    type Output = MessageType;

    #[tracing::instrument(name = "update_message_type_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let mut message_type = MessageType::find(ctx.tx(), MessageTypeQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve message type {}:{e}", cmd.id))?
            .ok_or(Error::MessageTypeNotFound(cmd.id))?;

        if cmd.name.is_none() & cmd.enabled.is_none() & cmd.vars.is_none() {
            return Ok(message_type);
        }

        if let Some(name) = cmd.name {
            message_type = message_type.set_name(name);
        }

        if let Some(enabled) = cmd.enabled {
            message_type = message_type.set_enabled(enabled);
        }

        if let Some(vars) = cmd.vars {
            message_type = message_type.set_vars(vars);
        }

        Ok(message_type
            .update(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to update message type {}: {e}", cmd.id))?)
    }
}
