use perroute_commons::types::{actor::Actor, id::Id, json_schema::JsonSchema};
use perroute_storage::{
    models::schema::{Schema, SchemasQueryBuilder},
    query::FetchableModel,
};

use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};

command!(
    UpdateSchemaCommand,
    CommandType::UpdateSchema,
    schema_id: Id,
    schema: JsonSchema
);
into_event!(UpdateSchemaCommand);

#[derive(Debug)]
pub struct UpdateSchemaCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for UpdateSchemaCommandHandler {
    type Command = UpdateSchemaCommand;
    type Output = Schema;

    #[tracing::instrument(name = "update_schema_handler", skip(self, ctx))]
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
