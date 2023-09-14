use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use anyhow::anyhow;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    id::Id,
    priority::Priority,
    properties::{Properties, PropertiesError},
};
use perroute_connectors::{types::dispatch_type::DispatchType, Plugins};
use perroute_storage::{
    models::channel::{Channel, ChannelQuery},
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::PgPool;
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

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct UpdateChannelCommand {
    id: Id,
    dispatch_properties: Option<Properties>,
    enabled: Option<bool>,
    priority: Option<Priority>,
}
impl_command!(UpdateChannelCommand, CommandType::UpdateChannel);
into_event!(UpdateChannelCommand);

#[derive(Debug)]
pub struct UpdateChannelCommandHandler {
    pool: PgPool,
    plugins: Plugins,
}

impl UpdateChannelCommandHandler {
    pub fn new(pool: PgPool, plugins: Plugins) -> Self {
        Self { pool, plugins }
    }
}

#[async_trait::async_trait]
impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;
    type Output = Channel;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext,

        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let mut channel = Channel::find(ctx.pool(), ChannelQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel: {e}"))?
            .ok_or_else(|| UpdateChannelError::ChannelNotFound(cmd.id))?;

        if cmd.dispatch_properties.is_none() && cmd.enabled.is_none() && cmd.priority.is_none() {
            return Ok(channel);
        }

        if let Some(props) = cmd.dispatch_properties {
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

            channel = channel.set_properties(props);
        }

        if let Some(enabled) = cmd.enabled {
            channel = channel.set_enabled(enabled);
        }

        if let Some(priority) = cmd.priority {
            channel = channel.set_priority(priority);
        }

        Ok(channel
            .update(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to update channel: {e}"))?)
    }
}
