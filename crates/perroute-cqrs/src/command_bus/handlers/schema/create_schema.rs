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
        json_schema::{JsonSchema, JsonSchemaError},
    },
};
use perroute_storage::{
    models::{
        message_type::{MessageType, MessageTypeQueryBuilder},
        schema::{Schema, SchemaBuilder},
    },
    query::FetchableModel,
};

command!(
    CreateSchemaCommand,
    CommandType::CreateSchema,
    schema_id: Id,
    message_type_id: Id,
    schema: JsonSchema
);
into_event!(CreateSchemaCommand);

#[derive(Debug, thiserror::Error)]
pub enum CreateSchemaError {
    #[error(transparent)]
    InvalidSchema(#[from] JsonSchemaError),
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
                .id(Some(*cmd.message_type_id()))
                .build()
                .unwrap(),
        )
        .await?
        .unwrap();
        let actual_version = Schema::max_version_number(ctx.tx(), mt.id()).await?;

        SchemaBuilder::default()
            .id(new_id!())
            //.schema(JsonSchema::try_from(cmd.schema().clone()).map_err(CreateSchemaError::from)?)
            .schema(cmd.schema().clone())
            .version(actual_version.increment())
            .published(false)
            .message_type_id(*mt.id())
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .map_err(CommandBusError::from)
    }
}
