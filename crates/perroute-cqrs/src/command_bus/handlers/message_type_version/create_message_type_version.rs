use crate::command_bus::{
    bus::CommandBusContext, commands::CreateMessageTypeVersionCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct CreateMessageTypeVersionCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for CreateMessageTypeVersionCommandHandler {
    type Command = CreateMessageTypeVersionCommand;

    async fn handle<'tx, 'a>(
        &self,
        _ctx: &mut CommandBusContext<'tx, 'a>,
        _cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        todo!()
    }
}
