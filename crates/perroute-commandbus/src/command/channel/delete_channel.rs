use crate::{bus::Ctx, command::Command, error::CommandBusError};
use perroute_commons::types::{actor::Actor, command_type::CommandType, id::Id};
use perroute_storage::{
    error::StorageError,
    models::{
        channel::{Channel, ChannelQuery},
        route::Route,
    },
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder)]
pub struct DeleteChannelCommand {
    id: Id,
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteChannelError {
    #[error("Channel {0} not found")]
    ChannelNotFound(Id),
}

#[async_trait::async_trait]
impl Command for DeleteChannelCommand {
    type Output = bool;

    async fn handle<'ctx>(&self, ctx: &mut Ctx<'ctx>) -> Result<Self::Output, CommandBusError> {
        let channel = Channel::find(ctx.pool(), ChannelQuery::with_id(self.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel {}: {e}", self.id))?
            .ok_or(DeleteChannelError::ChannelNotFound(self.id))?;

        let mut tx = ctx.pool().begin().await.map_err(StorageError::Tx)?;

        Route::delete_by_channel(tx.as_mut(), channel.id()).await?;

        let deleted = channel
            .delete(tx.as_mut())
            .await
            .tap_err(|e| tracing::error!("Failed to delete channel{}: {e}", self.id))?;

        tx.commit().await.map_err(StorageError::Tx)?;

        Ok(deleted)
    }

    fn command_type(&self) -> CommandType {
        CommandType::DeleteChannel
    }

    fn supports(&self, _actor: &Actor) -> bool {
        true
    }
}
