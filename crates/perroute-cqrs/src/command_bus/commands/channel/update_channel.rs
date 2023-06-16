use crate::command_bus::{
    bus::{Command, CommandBusContext, CommandBusError, CommandHandler},
    commands::CommandType,
};
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::id::Id;
use perroute_storage::models::channel::Channel;
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, new, Serialize, Clone, PartialEq, Eq)]
pub struct UpdateChannelCommand {
    pub chanel_id: Id,
    pub name: String,
}

impl Command for UpdateChannelCommand {
    fn ty(&self) -> CommandType {
        CommandType::UpdateChannel
    }
}

#[derive(Debug, new)]
pub struct UpdateChannelCommandHandler;

#[derive(thiserror::Error, Debug)]
pub enum UpdateChannelError {
    #[error("Channel with id {0} nor found")]
    ChannelNotFound(Id),
}

#[async_trait]
impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;

    #[tracing::instrument(skip(self))]
    async fn handle<'ctx>(
        &self,
        ctx: &mut crate::command_bus::bus::CommandBusContext<'ctx>,
        command: Self::Command,
    ) -> Result<(), CommandBusError> {
        let mut channel = retrieve_channel(ctx, command.chanel_id).await?;

        channel.with_name(command.name);

        channel.update(ctx.tx()).await.tap_err(|e| {
            tracing::error!("Error while updating channel {}: {e}", command.chanel_id)
        })?;

        Ok(())
    }
}

async fn retrieve_channel(
    ctx: &mut CommandBusContext<'_>,
    id: Id,
) -> Result<Channel, CommandBusError> {
    Channel::find_by_id(ctx.tx(), id)
        .await
        .tap_err(|e| tracing::error!("Error while retrieving channel {}: {e}", id))?
        .ok_or_else(|| UpdateChannelError::ChannelNotFound(id).into())
}
