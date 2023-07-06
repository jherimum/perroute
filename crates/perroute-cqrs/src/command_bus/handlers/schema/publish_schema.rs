use crate::command_bus::{
    bus::CommandBusContext, commands::PublishSchemaCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_storage::models::schema::Schema;

#[derive(Debug)]
pub struct PublishSchemaCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for PublishSchemaCommandHandler {
    type Command = PublishSchemaCommand;
    type Output = Schema;

    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        Schema::find_by_id(ctx.tx(), *cmd.schema_id())
            .await
            .unwrap()
            .unwrap()
            .set_published(true)
            .update(ctx.tx())
            .await
            .map_err(CommandBusError::from)
    }
}
