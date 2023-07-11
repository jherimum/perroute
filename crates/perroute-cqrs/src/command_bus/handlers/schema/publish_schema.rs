use crate::command_bus::{
    bus::CommandBusContext, commands::PublishSchemaCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::schema::{Schema, SchemasQueryBuilder},
    query::FetchableModel,
};

#[derive(Debug)]
pub struct PublishSchemaCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for PublishSchemaCommandHandler {
    type Command = PublishSchemaCommand;
    type Output = Schema;

    #[tracing::instrument(name = "publish_schema_handler", skip(self, ctx))]
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
        .set_published(true)
        .update(ctx.tx())
        .await
        .map_err(CommandBusError::from)
    }
}
