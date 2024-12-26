use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command,
};
use perroute_commons::{events::MessageTypeDeletedEvent, types::id::Id};
use perroute_storage::repository::{
    message_types::MessageTypeRepository, TransactedRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum DeleteMessageTypeCommandError {}

impl_command!(DeleteMessageTypeCommand, {
    message_type_id: Id,
});

pub struct DeleteMessageTypeCommandHandler;

impl CommandHandler for DeleteMessageTypeCommandHandler {
    type Command = DeleteMessageTypeCommand;
    type Output = ();
    type ApplicationEvent = MessageTypeDeletedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        let deleted = MessageTypeRepository::delete_message_type(
            ctx.repository(),
            &cmd.inner().message_type_id,
        )
        .await?;

        //Ok(())/
        todo!()
    }
}
