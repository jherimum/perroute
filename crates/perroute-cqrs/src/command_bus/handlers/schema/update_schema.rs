use perroute_storage::models::schema::Schema;

use crate::command_bus::{
    bus::CommandBusContext, commands::UpdateSchemaCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct UpdateSchemaCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for UpdateSchemaCommandHandler {
    type Command = UpdateSchemaCommand;

    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        let mtv = Schema::find_by_id(ctx.tx(), cmd.schema_id())
            .await?
            .unwrap();

        Ok(())
    }
}
