pub mod create_channel;
pub mod delete_channel;
pub mod update_channel;

use crate::command_bus::{bus::CommandBusContext, error::CommandBusError};
use perroute_commons::types::id::Id;
use perroute_storage::models::channel::Channel;
use tap::TapFallible;

pub async fn retrieve_channel<'ctx>(
    ctx: &mut CommandBusContext<'ctx>,
    id: &Id,
    to_error: impl FnOnce(Id) -> CommandBusError,
) -> Result<Channel, CommandBusError> {
    match Channel::find_by_id(ctx.tx(), id)
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve channel {}: {e}", id))?
    {
        Some(channel) => Ok(channel),
        None => Err(to_error(*id)),
    }
}
