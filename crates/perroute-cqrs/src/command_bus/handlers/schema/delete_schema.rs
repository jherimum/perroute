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
    models::{
        route::{Route, RouteQuery},
        schema::{Schema, SchemasQuery},
        template::{Template, TemplatesQuery},
    },
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Schema with id {0} not found")]
    SchemaNotFound(Id),

    #[error("Schema {0} could not be deleted: {1}")]
    SchemaDelete(Id, &'static str),
}

command!(
    DeleteSchemaCommand,
    CommandType::DeleteSchema,
    id: Id
);
into_event!(DeleteSchemaCommand);

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
        let schema = Schema::find(ctx.pool(), SchemasQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::info!("Faled to retrieve schema: {e}"))?
            .ok_or(Error::SchemaNotFound(cmd.id))?;

        if schema.exists_messages(ctx.pool()).await? {
            return Err(
                Error::SchemaDelete(cmd.id, "There are messages associated with message").into(),
            );
        }

        let template_ids = Template::ids(ctx.pool(), TemplatesQuery::with_schema_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve templates:{e}"))?;

        Template::batch_delete(template_ids, ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to delete templates:{e}"))?;

        let route_ids = Route::ids(ctx.pool(), RouteQuery::with_schema_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve routes:{e}"))?;

        Route::batch_delete(route_ids, ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to delete routes:{e}"))?;

        Ok(schema
            .delete(ctx.tx())
            .await
            .tap_err(|e| tracing::info!("Failed to delete schema: {e}"))?)
    }
}
