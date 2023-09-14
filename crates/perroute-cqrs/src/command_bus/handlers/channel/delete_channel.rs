use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::id::Id;
use perroute_storage::{
    error::StorageError,
    models::{
        channel::{Channel, ChannelQuery},
        route::Route,
    },
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct DeleteChannelCommand {
    id: Id,
}
impl_command!(DeleteChannelCommand, CommandType::DeleteChannel);
into_event!(DeleteChannelCommand);

#[derive(Debug, thiserror::Error)]
pub enum DeleteChannelError {
    #[error("Channel {0} not found")]
    ChannelNotFound(Id),
}

#[derive(Debug)]
pub struct DeleteChannelCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for DeleteChannelCommandHandler {
    type Command = DeleteChannelCommand;
    type Output = bool;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext,

        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let channel = Channel::find(ctx.pool(), ChannelQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel {}: {e}", cmd.id))?
            .ok_or(DeleteChannelError::ChannelNotFound(cmd.id))?;

        let mut tx = ctx.pool().begin().await.map_err(StorageError::Tx)?;

        Route::delete_by_channel(&mut tx, channel.id()).await?;

        let deleted = channel
            .delete(&mut tx)
            .await
            .tap_err(|e| tracing::error!("Failed to delete channel{}: {e}", cmd.id))?;

        tx.commit().await.map_err(StorageError::Tx)?;

        Ok(deleted)
    }
}
