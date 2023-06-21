use crate::command_bus::{
    bus::{CommandBusContext, CommandHandler},
    commands::CreateChannelCommand,
    error::CommandBusError,
};
use async_trait::async_trait;
use perroute_commons::types::code::Code;
use perroute_storage::models::channel::{Channel, ChannelBuilder};
use tap::TapFallible;

#[derive(Debug)]
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
        if Channel::exists_by_code(ctx.tx(), cmd.code())
            .await
            .tap_err(|e| {
                tracing::error!(
                    "Failed to checking if channel with code {} exists: {e}",
                    cmd.code()
                )
            })?
        {
            tracing::error!("Channel with code {} already exists", cmd.code());
            return Err(CreateChannelError::CodeAlreadyExists(cmd.code().clone()).into());
        }

        ChannelBuilder::default()
            .id(*cmd.channel_id())
            .code(cmd.code().clone())
            .name(cmd.name().clone())
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .unwrap();

        Ok(())
    }
}
