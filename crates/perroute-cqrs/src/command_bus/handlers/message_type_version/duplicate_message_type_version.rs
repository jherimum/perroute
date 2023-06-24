use crate::command_bus::{
    bus::CommandBusContext, commands::DuplicateMessageTypeVersionCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct DuplicateMessageTypeVersionCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for DuplicateMessageTypeVersionCommandHandler {
    type Command = DuplicateMessageTypeVersionCommand;

    async fn handle<'tx, 'a>(
        &self,
        _ctx: &mut CommandBusContext<'tx, 'a>,
        _cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        todo!()
    }
}
