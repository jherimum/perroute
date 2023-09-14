use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    into_event,
};
use anyhow::Context;
use derive_getters::Getters;
use perroute_commons::{
    new_id,
    types::{
        id::Id,
        json_schema::{InvalidSchemaError, JsonSchema},
        vars::Vars,
    },
};
use perroute_storage::{
    models::{
        message_type::{MessageType, MessageTypeQuery},
        schema::{Schema, SchemaBuilder},
    },
    query::FetchableModel,
};
use sqlx::PgPool;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateSchemaError {
    #[error(transparent)]
    InvalidSchema(#[from] InvalidSchemaError),

    #[error("Message type with id {0} not found")]
    MessageTypeNotFound(Id),
}

command!(
    CreateSchemaCommand,
    CommandType::CreateSchema,
    id: Id,
    message_type_id: Id,
    value: JsonSchema,
    vars: Vars
);
into_event!(CreateSchemaCommand);

#[derive(Debug, Getters)]
pub struct CreateSchemaCommandHandler {
    pool: PgPool,
}

impl CreateSchemaCommandHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CommandHandler for CreateSchemaCommandHandler {
    type Command = CreateSchemaCommand;
    type Output = Schema;

    #[tracing::instrument(name = "create_schema_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext,

        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let mt = MessageType::find(ctx.pool(), MessageTypeQuery::with_id(cmd.message_type_id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve message type: {e}"))?
            .ok_or(CreateSchemaError::MessageTypeNotFound(cmd.message_type_id))?;

        let next_version = Schema::next_version(ctx.pool(), mt.id())
            .await
            .tap_err(|e| tracing::error!("Failed to calculate next version number: {e}"))?;

        Ok(SchemaBuilder::default()
            .id(new_id!())
            .value(cmd.value)
            .version(next_version)
            .published(false)
            .message_type_id(cmd.message_type_id)
            .business_unit_id(*mt.business_unit_id())
            .enabled(false)
            .vars(cmd.vars)
            .build()
            .context("Failed to build schema")?
            .save(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to save schema: {e}"))?)
    }
}
