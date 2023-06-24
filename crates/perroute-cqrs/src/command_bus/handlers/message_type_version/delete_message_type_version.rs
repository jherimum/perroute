use crate::command_bus::{
    bus::CommandBusContext, commands::DeleteMessageTypeVersionCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct DeleteMessageTypeVersionCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for DeleteMessageTypeVersionCommandHandler {
    type Command = DeleteMessageTypeVersionCommand;

    async fn handle<'tx, 'a>(
        &self,
        _ctx: &mut CommandBusContext<'tx, 'a>,
        _cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        todo!()
    }
}
