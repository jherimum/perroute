use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use perroute_commons::{
    new_id,
    types::{
        actor::Actor,
        id::Id,
        json_schema::{InvalidSchemaError, JsonSchema},
        vars::Vars,
    },
};
use perroute_storage::{
    models::{
        message_type::{MessageType, MessageTypeQueryBuilder},
        schema::{Schema, SchemaBuilder},
    },
    query::FetchableModel,
};
use tap::{TapFallible, TapOptional};

command!(
    CreateSchemaCommand,
    CommandType::CreateSchema,
    id: Id,
    message_type_id: Id,
    value: JsonSchema,
    vars: Vars
);
into_event!(CreateSchemaCommand);

#[derive(Debug, thiserror::Error)]
pub enum CreateSchemaError {
    #[error(transparent)]
    InvalidSchema(#[from] InvalidSchemaError),
}

#[derive(Debug)]
pub struct CreateSchemaCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for CreateSchemaCommandHandler {
    type Command = CreateSchemaCommand;
    type Output = Schema;

    #[tracing::instrument(name = "create_schema_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let mt = MessageType::find(
            ctx.tx(),
            MessageTypeQueryBuilder::default()
                .id(Some(cmd.message_type_id))
                .build()
                .unwrap(),
        )
        .await?
        .tap_none(|| tracing::error!("message type not found"))
        .unwrap();

        let actual_version = Schema::max_version_number(ctx.tx(), mt.id())
            .await
            .tap_err(|e| tracing::error!("Failed to calculate next version number: {e}"))?;

        SchemaBuilder::default()
            .id(new_id!())
            .value(cmd.value)
            .version(actual_version.increment())
            .published(false)
            .message_type_id(*mt.id())
            .enabled(false)
            .vars(cmd.vars)
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to save schema: {e}"))
            .map_err(CommandBusError::from)
    }
}
