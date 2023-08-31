use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, code::Code, id::Id, vars::Vars};
use perroute_storage::{
    models::message_type::{MessageType, MessageTypeBuilder, MessageTypeQuery},
    query::FetchableModel,
};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateMessageTypeError {
    #[error("Code {0} already exists")]
    CodeAlreadyExists(Code),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateMessageTypeCommand {
    #[builder(default)]
    id: Id,
    code: Code,
    name: String,
    vars: Vars,
    business_unit_id: Id,
}

impl_command!(CreateMessageTypeCommand, CommandType::CreateMessageType);
into_event!(CreateMessageTypeCommand);

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
    ) -> Result<MessageType> {
        if MessageType::exists(
            ctx.pool(),
            MessageTypeQuery::with_code_and_business_unit(cmd.code.clone(), cmd.business_unit_id),
        )
        .await?
        {
            return Err(CreateMessageTypeError::CodeAlreadyExists(cmd.code().clone()).into());
        }

        let message_type = MessageTypeBuilder::default()
            .id(cmd.id)
            .code(cmd.code)
            .name(cmd.name)
            .enabled(false)
            .vars(cmd.vars)
            .business_unit_id(cmd.business_unit_id)
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to save message type: {e}"))?;
        Ok(message_type)
    }
}
