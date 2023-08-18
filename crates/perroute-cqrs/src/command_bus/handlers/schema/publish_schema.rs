use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::schema::{Schema, SchemasQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Schema with id {0} not found")]
    SchemaNotFound(Id),
}

command!(
    PublishSchemaCommand,
    CommandType::PublishSchema,
    id: Id
);
into_event!(PublishSchemaCommand);

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
        let schema = Schema::find(ctx.tx(), SchemasQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::info!("Faled to retrieve schema: {e}"))?
            .ok_or(Error::SchemaNotFound(cmd.id))?;

        if *schema.published() {
            return Ok(schema);
        }
        Ok(schema
            .set_published(true)
            .update(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to update schema:{e}"))?)
    }
}
