use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
};
use bon::Builder;
use perroute_commons::types::id::Id;
use perroute_storage::repository::{message_types::MessageTypeRepository, TransactedRepository};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum DeleteMessageTypeCommandError {}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct DeleteMessageTypeCommand {
    id: Id,
}

impl Command for DeleteMessageTypeCommand {
    fn event_type(&self) -> perroute_commons::events::EventType {
        perroute_commons::events::EventType::MessageTypeDeleted
    }

    fn entity_id(&self) -> &Id {
        todo!()
    }
}

pub struct DeleteMessageTypeCommandHandler;

impl CommandHandler for DeleteMessageTypeCommandHandler {
    type Command = DeleteMessageTypeCommand;
    type Output = ();

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let deleted =
            MessageTypeRepository::delete_message_type(ctx.repository(), &cmd.inner().id).await?;

        Ok(())
    }
}
