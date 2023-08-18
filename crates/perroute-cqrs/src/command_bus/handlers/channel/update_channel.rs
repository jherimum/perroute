use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use anyhow::anyhow;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor,
    id::Id,
    priority::Priority,
    properties::{Properties, PropertiesError},
};
use perroute_connectors::types::DispatchType;
use perroute_storage::{
    models::channel::{Channel, ChannelQuery},
    query::FetchableModel,
};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Channel {0} nor found")]
    ChannelNotFound(Id),

    #[error("Dispatch type  {0} not supoorted by plugin")]
    DispatchTypeNotSupported(DispatchType),

    #[error("Invalid properties")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct UpdateChannelCommand {
    id: Id,
    dispatch_properties: Properties,
    enabled: bool,
    priority: Priority,
}
impl_command!(UpdateChannelCommand, CommandType::UpdateChannel);
into_event!(UpdateChannelCommand);

#[derive(Debug)]
pub struct UpdateChannelCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;
    type Output = Channel;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let channel = Channel::find(ctx.pool(), ChannelQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel: {e}"))?
            .ok_or_else(|| Error::ChannelNotFound(cmd.id))?;

        let conn = channel
            .connection(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve connection: {e}"))?;

        let plugin = ctx
            .plugins()
            .get(conn.plugin_id())
            .ok_or_else(|| anyhow!("Plugin not found"))?;

        let disp = plugin
            .dispatcher(channel.dispatch_type())
            .ok_or_else(|| anyhow!("Dispatcher type not supported"))?;

        disp.configuration()
            .validate(cmd.dispatch_properties())
            .map_err(Error::from)?;

        Ok(channel
            .set_enabled(cmd.enabled)
            .set_priority(cmd.priority)
            .set_properties(cmd.dispatch_properties)
            .update(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to update channel: {e}"))?)
    }
}
