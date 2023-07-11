use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::schema::{Schema, SchemasQueryBuilder},
    query::FetchableModel,
};

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

    #[tracing::instrument(name = "delete_schema_handler", skip(self, ctx))]
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
        .await
        .unwrap()
        .unwrap()
        .delete(ctx.tx())
        .await
        .map_err(CommandBusError::from)
    }
}
