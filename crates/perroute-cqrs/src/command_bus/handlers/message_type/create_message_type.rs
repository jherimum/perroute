use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, code::Code, id::Id, vars::Vars};
use perroute_storage::{
    models::message_type::{MessageType, MessageTypeBuilder, MessageTypeQueryBuilder},
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::types::Json;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateMessageTypeCommand {
    #[builder(default)]
    message_type_id: Id,
    code: Code,
    name: String,
    enabled: bool,
    vars: Vars,
    bu_id: Id,
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
            .name(cmd.name().clone())
            .enabled(false)
            .vars(Json(cmd.vars().clone()))
            .business_id(cmd.bu_id)
            .build()
            .unwrap()
            .save(ctx.tx())
            .await?;
        Ok(message_type)
    }
}
