use crate::command_bus::{
    bus::CommandBusContext, commands::PublishMessageTypeVersionCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct PublishMessageTypeVersionCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for PublishMessageTypeVersionCommandHandler {
    type Command = PublishMessageTypeVersionCommand;

    async fn handle<'tx, 'a>(
        &self,
        _ctx: &mut CommandBusContext<'tx, 'a>,
        _cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        todo!()
    }
}
