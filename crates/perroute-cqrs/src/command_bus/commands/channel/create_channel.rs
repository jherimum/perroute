use crate::command_bus::{
    bus::{Command, CommandBusContext, CommandHandler},
    commands::CommandType,
    error::CommandBusError,
};
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::{code::Code, id::Id};
use perroute_storage::models::channel::Channel;
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, new, Serialize, Clone, PartialEq, Eq)]
pub struct CreateChannelCommand {
    pub channel_id: Id,
    pub code: Code,
    pub name: String,
}

impl Command for CreateChannelCommand {
    fn ty(&self) -> CommandType {
        CommandType::CreateChannel
    }
}

#[derive(Debug, new)]
pub struct CreateChannelCommandHandler;

#[derive(thiserror::Error, Debug)]
pub enum CreateChannelError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error("A channel with code {0} already exists")]
    CodeAlreadyExists(Code),
}

#[async_trait]
impl CommandHandler for CreateChannelCommandHandler {
    type Command = CreateChannelCommand;

    #[tracing::instrument(skip(self))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        if Channel::exists_by_code(ctx.tx(), cmd.code.clone())
            .await
            .tap_err(|e| {
                tracing::error!(
                    "Failed to checking if channel with code {} exists: {e}",
                    cmd.code
                )
            })?
        {
            return Err(CreateChannelError::CodeAlreadyExists(cmd.code.clone()).into());
        }

        let channel = Channel::new(cmd.channel_id, cmd.code, &cmd.name)
            .save(ctx.tx())
            .await
            .unwrap();

        // Ok(Event::ChannelEvent(crate::commands::ChannelEvent::Created(
        //     channel.id().clone(),
        // )))
        Ok(())
    }
}
