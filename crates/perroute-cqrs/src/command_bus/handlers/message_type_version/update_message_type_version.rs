use crate::command_bus::{
    bus::CommandBusContext, commands::UpdateMessageTypeVersionCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct UpdateMessageTypeVersionCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for UpdateMessageTypeVersionCommandHandler {
    type Command = UpdateMessageTypeVersionCommand;

    async fn handle<'tx, 'a>(
        &self,
        _ctx: &mut CommandBusContext<'tx, 'a>,
        _cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        todo!()
    }
}
