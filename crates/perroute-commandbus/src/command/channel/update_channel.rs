use crate::{bus::Ctx, command::Command, error::CommandBusError};
use anyhow::anyhow;
use perroute_commons::types::{
    actor::Actor,
    command_type::CommandType,
    id::Id,
    properties::{Properties, PropertiesError},
};
use perroute_connectors::types::dispatch_type::DispatchType;
use perroute_storage::{
    models::channel::{Channel, ChannelQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum UpdateChannelError {
    #[error("Channel {0} nor found")]
    ChannelNotFound(Id),

    #[error("Dispatch type  {0} not supoorted by plugin")]
    DispatchTypeNotSupported(DispatchType),

    #[error("Invalid properties")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug)]
pub struct UpdateChannelCommand {
    id: Id,
    dispatch_properties: Option<Properties>,
    enabled: Option<bool>,
}

#[async_trait::async_trait]
impl Command for UpdateChannelCommand {
    type Output = Channel;

    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        let mut channel = Channel::find(ctx.pool(), ChannelQuery::with_id(self.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel: {e}"))?
            .ok_or_else(|| UpdateChannelError::ChannelNotFound(self.id))?;

        if self.dispatch_properties.is_none() && self.enabled.is_none() {
            return Ok(channel);
        }

        if let Some(props) = &self.dispatch_properties {
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
                .validate(&props)
                .map_err(UpdateChannelError::from)?;

            channel = channel.set_properties(props.clone());
        }

        if let Some(enabled) = self.enabled {
            channel = channel.set_enabled(enabled);
        }

        Ok(channel
            .update(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to update channel: {e}"))?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::UpdateChannel
    }

    fn supports(&self, _actor: &Actor) -> bool {
        true
    }
}
