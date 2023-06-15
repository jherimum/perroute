use crate::{
    command_bus::{Command, CommandBusContext, CommandBusError, CommandHandler},
    commands::CommandType,
};
use anyhow::Context;
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::code::Code;
use perroute_storage::models::{channel::Channel, command_log::CommandLog};
use serde::Serialize;

#[derive(Debug, new, Serialize, Clone, PartialEq, Eq)]
pub struct CreateChannelCommand {
    code: Code,
    name: String,
}

impl Command for CreateChannelCommand {
    fn ty(&self) -> CommandType {
        CommandType::CreateChannel
    }
}

impl From<CommandLog<Self>> for CreateChannelCommand {
    fn from(value: CommandLog<Self>) -> Self {
        todo!()
    }
}

#[derive(Debug, new)]
pub struct Handler;

#[derive(thiserror::Error, Debug)]
pub enum CreateChannelError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error("A channel with code {0} already exists")]
    CodeAlreadyExists(Code),
}

#[async_trait]
impl CommandHandler for Handler {
    type Command = CreateChannelCommand;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        cmd: &Self::Command,
    ) -> Result<(), CommandBusError> {
        if Channel::exists_by_code(ctx.pool(), &cmd.code)
            .await
            .with_context(|| {
                format!(
                    "Error while checking if channel with code {} exists",
                    cmd.code,
                )
            })?
        {
            return Err(CreateChannelError::CodeAlreadyExists(cmd.code.clone()).into());
        }

        let channel = Channel::new(&cmd.code, &cmd.name)
            .save(ctx.tx())
            .await
            .unwrap();

        // Ok(Event::ChannelEvent(crate::commands::ChannelEvent::Created(
        //     channel.id().clone(),
        // )))
        Ok(())
    }
}
