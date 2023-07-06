use perroute_storage::models::schema::Schema;

use crate::command_bus::{
    bus::CommandBusContext, commands::DeleteSchemaCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct DeleteSchemaCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for DeleteSchemaCommandHandler {
    type Command = DeleteSchemaCommand;
    type Output = bool;

    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        Schema::find_by_id(ctx.tx(), *cmd.schema_id())
            .await
            .unwrap()
            .unwrap()
            .delete(ctx.tx())
            .await
            .map_err(CommandBusError::from)
    }
}
