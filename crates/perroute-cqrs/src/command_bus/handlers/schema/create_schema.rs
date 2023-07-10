use crate::command_bus::{
    bus::CommandBusContext, commands::CreateSchemaCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::{new_id, types::json_schema::JsonSchemaError};
use perroute_storage::{
    models::{
        message_type::{MessageType, MessageTypeQueryBuilder},
        schema::{Schema, SchemaBuilder},
    },
    query::FetchableModel,
};

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

    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
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
            .channel_id(*mt.channel_id())
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .map_err(CommandBusError::from)
    }
}
