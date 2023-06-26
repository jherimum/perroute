use crate::command_bus::{
    bus::CommandBusContext, commands::CreateSchemaCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::new_id;
use perroute_storage::models::{
    message_type::MessageType,
    schema::{Schema, SchemaBuilder},
};

#[derive(Debug)]
pub struct CreateSchemaCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for CreateSchemaCommandHandler {
    type Command = CreateSchemaCommand;

    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        let mt = MessageType::find_by_id(ctx.tx(), cmd.message_type_id())
            .await?
            .unwrap();
        let actual_version = Schema::max_version_number(ctx.tx(), mt.id()).await?;

        SchemaBuilder::default()
            .id(new_id!())
            .schema(cmd.schema().clone())
            .version(actual_version.increment())
            .published(false)
            .message_type_id(*mt.id())
            .channel_id(*mt.channel_id())
            .build()
            .unwrap()
            .save(ctx.tx())
            .await?;
        Ok(())
    }
}
