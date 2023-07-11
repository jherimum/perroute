use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::schema::{Schema, SchemasQueryBuilder},
    query::FetchableModel,
};

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

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        Schema::find(
            ctx.tx(),
            SchemasQueryBuilder::default()
                .id(Some(*cmd.schema_id()))
                .build()
                .unwrap(),
        )
        .await?
        .unwrap()
        .set_schema(cmd.schema().clone())
        .update(ctx.tx())
        .await
        .map_err(CommandBusError::from)
    }
}
