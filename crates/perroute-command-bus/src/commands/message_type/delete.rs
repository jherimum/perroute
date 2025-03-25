use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command,
};
use perroute_commons::{events::MessageTypeDeletedEvent, types::id::Id};

#[derive(Debug, thiserror::Error)]
pub enum DeleteMessageTypeCommandError {}

impl_command!(DeleteMessageTypeCommand, {
    message_type_id: Id,
});

pub struct DeleteMessageTypeCommandHandler;

impl CommandHandler for DeleteMessageTypeCommandHandler {
    type Command = DeleteMessageTypeCommand;
    type Output = ();
    type E = MessageTypeDeletedEvent;

    async fn handle<
        C: AsRef<perroute_storage::active_record::datasource::Connection>,
    >(
        &self,
        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> CommandHandlerResult<Self::Output> {
        // let deleted = MessageTypeRepository::delete_message_type(
        //     ctx.repository(),
        //     &cmd.inner().message_type_id,
        // )
        // .await?;

        // //Ok(())/
        todo!()
    }
}
