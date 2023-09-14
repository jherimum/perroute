use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    into_event,
};
use perroute_commons::types::{id::Id, json_schema::JsonSchema, vars::Vars};
use perroute_storage::{
    models::schema::{Schema, SchemasQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum UpdateSchemaError {
    #[error("Schema with id {0} not found")]
    SchemaNotFound(Id),

    #[error("Schema {0} already published")]
    SchemaAlreadyPublished(Id),
}

command!(
    UpdateSchemaCommand,
    CommandType::UpdateSchema,
    id: Id,
    value: Option<JsonSchema>,
    enabled: Option<bool>,
    vars: Option<Vars>
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
        ctx: &mut CommandBusContext,

        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let mut schema = Schema::find(ctx.pool(), SchemasQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve schema {}:{e}", cmd.id))?
            .ok_or(UpdateSchemaError::SchemaNotFound(cmd.id))?;

        if cmd.enabled.is_none() & cmd.vars.is_none() & cmd.value.is_none() {
            return Ok(schema);
        }

        if let Some(cmd_value) = cmd.value {
            schema = schema.set_value(cmd_value);
        }

        if let Some(enabled) = cmd.enabled {
            schema = schema.set_enabled(enabled);
        }

        if let Some(vars) = cmd.vars {
            schema = schema.set_vars(vars);
        }

        Ok(schema
            .update(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to update schema:{e}"))?)
    }
}
