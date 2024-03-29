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
    models::message_type::{MessageType, MessageTypeQueryBuilder},
    query::FetchableModel,
};

command!(
    UpdateMessageTypeCommand,
    CommandType::UpdateMessageType,
    id: Id,
    name: String,
    enabled: bool,
    vars: Vars

);
into_event!(UpdateMessageTypeCommand);

#[derive(Debug, thiserror::Error)]
pub enum UpdateMessageTypeError {
    #[error("Message type with id {0} not found")]
    MessageTypeNotFound(Id),
}

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
        MessageType::find(
            ctx.tx(),
            MessageTypeQueryBuilder::default()
                .id(Some(cmd.id))
                .build()
                .unwrap(),
        )
        .await?
        .ok_or(UpdateMessageTypeError::MessageTypeNotFound(cmd.id))?
        .set_name(cmd.name)
        .set_enabled(cmd.enabled)
        .set_vars(cmd.vars)
        .update(ctx.tx())
        .await
        .map_err(CommandBusError::from)
    }
}
