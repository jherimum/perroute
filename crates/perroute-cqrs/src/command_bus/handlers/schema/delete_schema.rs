use crate::command_bus::{
    bus::CommandBusContext, commands::DeleteSchemaCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct DeleteSchemaCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for DeleteSchemaCommandHandler {
    type Command = DeleteSchemaCommand;

    async fn handle<'tx, 'a>(
        &self,
        _ctx: &mut CommandBusContext<'tx, 'a>,
        _cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        todo!()
    }
}
