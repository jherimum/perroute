use crate::command_bus::{
    bus::{Command, CommandBusContext, CommandHandler},
    commands::CommandType,
    error::CommandBusError,
};
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::{code::Code, id::Id};
use perroute_storage::models::channel::{Channel, ChannelBuilder};
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

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateChannelError {
    #[error("A channel with code {0} already exists")]
    CodeAlreadyExists(Code),
}

#[async_trait]
impl CommandHandler for CreateChannelCommandHandler {
    type Command = CreateChannelCommand;

    #[tracing::instrument(name = "create_channel_command", skip(self))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        if Channel::exists_by_code(ctx.tx(), &cmd.code)
            .await
            .tap_err(|e| {
                tracing::error!(
                    "Failed to checking if channel with code {} exists: {e}",
                    cmd.code
                )
            })?
        {
            tracing::error!("Channel with code {} already exists", cmd.code);
            return Err(CreateChannelError::CodeAlreadyExists(cmd.code.clone()).into());
        }

        ChannelBuilder::default()
            .id(cmd.channel_id)
            .code(cmd.code)
            .name(cmd.name)
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .unwrap();

        Ok(())
    }
}
