use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use perroute_commons::types::{actor::Actor, id::Id, json_schema::JsonSchema, vars::Vars};
use perroute_storage::{
    models::schema::{Schema, SchemasQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Schema with id {0} not found")]
    SchemaNotFound(Id),

    #[error("Schema {0} already published")]
    SchemaAlreadyPublished(Id),
}

command!(
    UpdateSchemaCommand,
    CommandType::UpdateSchema,
    id: Id,
    value: JsonSchema,
    enabled: bool,
    vars: Vars
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
        let schema = Schema::find(ctx.tx(), SchemasQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve schema {}:{e}", cmd.id))?
            .ok_or(Error::SchemaNotFound(cmd.id))?;

        //todo: fazer validacoes de published para nao alterar o schema

        Ok(schema
            .set_value(cmd.value)
            .set_enabled(cmd.enabled)
            .set_vars(cmd.vars)
            .update(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to update schema:{e}"))?)
    }
}
