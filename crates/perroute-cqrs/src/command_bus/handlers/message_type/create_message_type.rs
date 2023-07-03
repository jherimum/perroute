use crate::command_bus::{
    bus::CommandBusContext, commands::CreateMessageTypeCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::{
    new_id,
    types::{code::Code, json_schema::JsonSchema},
};
use perroute_storage::models::{
    message_type::{MessageType, MessageTypeBuilder},
    schema::SchemaBuilder,
    schema::Version,
};

#[derive(Debug, thiserror::Error)]
pub enum CreateMessageTypeError {
    #[error("Code {0} already exists")]
    CodeAlreadyExists(Code),
}

#[derive(Debug)]
pub struct CreateMessageTypeCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for CreateMessageTypeCommandHandler {
    type Command = CreateMessageTypeCommand;
    type Output = MessageType;

    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<MessageType, CommandBusError> {
        if MessageType::exists_code(ctx.tx(), cmd.channel_id(), cmd.code()).await? {
            return Err(CreateMessageTypeError::CodeAlreadyExists(cmd.code().clone()).into());
        }

        let message_type = MessageTypeBuilder::default()
            .id(*cmd.message_type_id())
            .code(cmd.code().clone())
            .description(cmd.description().clone())
            .enabled(false)
            .channel_id(*cmd.channel_id())
            .build()
            .unwrap()
            .save(ctx.tx())
            .await?;

        SchemaBuilder::default()
            .id(new_id!())
            .schema(JsonSchema::default())
            .version(Version::default())
            .message_type_id(*cmd.message_type_id())
            .published(false)
            .build()
            .unwrap()
            .save(ctx.tx())
            .await?;

        Ok(message_type)
    }
}
