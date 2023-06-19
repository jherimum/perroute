use super::retrieve_channel;
use crate::command_bus::{
    bus::{Command, CommandBusContext, CommandHandler},
    commands::CommandType,
    error::CommandBusError,
};
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::id::Id;
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, new, Serialize, Clone, PartialEq, Eq)]
pub struct DeleteChannelCommand {
    pub channel_id: Id,
}

impl Command for DeleteChannelCommand {
    fn ty(&self) -> CommandType {
        CommandType::DeleteChannel
    }
}

#[derive(Debug, new)]
pub struct DeleteChannelCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum DeleteChannelError {
    #[error("Channel with id {0} nor found")]
    ChannelNotFound(Id),
}

#[async_trait]
impl CommandHandler for DeleteChannelCommandHandler {
    type Command = DeleteChannelCommand;

    #[tracing::instrument(skip(self))]
    async fn handle<'ctx>(
        &self,
        ctx: &mut CommandBusContext<'ctx>,
        command: Self::Command,
    ) -> Result<(), CommandBusError> {
        retrieve_channel(ctx, command.channel_id, |id| {
            DeleteChannelError::ChannelNotFound(id).into()
        })
        .await?
        .delete(ctx.tx())
        .await
        .tap_err(|e| tracing::error!("Failed to delete channel {}: {e}", command.channel_id))?;

        Ok(())
    }
}
