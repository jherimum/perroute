use perroute_commons::types::json_schema::JsonSchema;
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
    type Output = Schema;

    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        Schema::find_by_id(ctx.tx(), *cmd.schema_id())
            .await?
            .unwrap()
            .set_schema(JsonSchema::try_from(cmd.schema().clone()).unwrap())
            .update(ctx.tx())
            .await
            .map_err(CommandBusError::from)
    }
}
