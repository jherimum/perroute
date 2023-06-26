use crate::command_bus::{
    bus::CommandBusContext, commands::PublishSchemaCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct PublishSchemaCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for PublishSchemaCommandHandler {
    type Command = PublishSchemaCommand;

    async fn handle<'tx, 'a>(
        &self,
        _ctx: &mut CommandBusContext<'tx, 'a>,
        _cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        todo!()
    }
}
