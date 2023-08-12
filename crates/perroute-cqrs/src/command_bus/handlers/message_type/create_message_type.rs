use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::{
    new_id,
    types::{actor::Actor, code::Code, id::Id, json_schema::JsonSchema},
};
use perroute_storage::{
    models::{
        message_type::{MessageType, MessageTypeBuilder, MessageTypeQueryBuilder},
        schema::SchemaBuilder,
        schema::Version,
    },
    query::FetchableModel,
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateMessageTypeCommand {
    #[builder(default)]
    message_type_id: Id,
    code: Code,
    description: String,
    channel_id: Id,
}

impl_command!(CreateMessageTypeCommand, CommandType::CreateMessageType);
into_event!(CreateMessageTypeCommand);

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

    #[tracing::instrument(name = "create_message_type_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<MessageType, CommandBusError> {
        if MessageType::exists(
            ctx.pool(),
            MessageTypeQueryBuilder::default()
                .channel_id(Some(*cmd.channel_id()))
                .code(Some(cmd.code().clone()))
                .build()
                .unwrap(),
        )
        .await?
        {
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
            .channel_id(*message_type.channel_id())
            .published(false)
            .build()
            .unwrap()
            .save(ctx.tx())
            .await?;

        Ok(message_type)
    }
}
