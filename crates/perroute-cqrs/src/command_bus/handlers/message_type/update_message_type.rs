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
    name: String,
    enabled: bool,
    vars: Vars

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
        Ok(
            MessageType::find(ctx.tx(), MessageTypeQuery::with_id(cmd.id))
                .await
                .tap_err(|e| tracing::error!("Failed to retrieve message type {}:{e}", cmd.id))?
                .ok_or(Error::MessageTypeNotFound(cmd.id))?
                .set_name(cmd.name)
                .set_enabled(cmd.enabled)
                .set_vars(cmd.vars)
                .update(ctx.tx())
                .await
                .tap_err(|e| tracing::error!("Failed to update message type {}: {e}", cmd.id))?,
        )
    }
}
