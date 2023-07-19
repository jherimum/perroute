use crate::command_bus::{
    bus::CommandBusContext, commands::CreateChannelCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, code::Code};
use perroute_storage::{
    models::channel::{Channel, ChannelBuilder, ChannelsQueryBuilder},
    query::FetchableModel,
};
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
    type Output = Channel;

    #[tracing::instrument(name = "create_channel_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        if Channel::exists(
            ctx.tx(),
            ChannelsQueryBuilder::default()
                .code(Some(cmd.code().clone()))
                .build()
                .unwrap(),
        )
        .await
        .tap_err(|e| {
            tracing::error!(
                "Failed to checking if channel with code {} exists: {e}",
                cmd.code()
            );
        })? {
            return Err(CommandBusError::ExpectedError(
                "Channel with code already exists",
            ));
        }

        ChannelBuilder::default()
            .id(*cmd.channel_id())
            .code(cmd.code().clone())
            .name(cmd.name().clone())
            .build()
            .tap_err(|e| {
                tracing::error!("Failed to build channel: {e}");
            })
            .map_err(|_| CommandBusError::UnexpectedError("Failed to build channel"))?
            .save(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to save channel: {e}"))
            .map_err(CommandBusError::from)
    }
}
