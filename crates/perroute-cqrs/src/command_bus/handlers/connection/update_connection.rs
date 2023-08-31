use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use anyhow::Context;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor,
    id::Id,
    properties::{Properties, PropertiesError},
};
use perroute_storage::{
    models::connection::{Connection, ConnectionQuery},
    query::FetchableModel,
};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum UpdateConnectionError {
    #[error("Connection with id {0} not found")]
    ConnectionNotFound(Id),

    #[error("Invalid properties: {0}")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct UpdateConnectionCommand {
    id: Id,
    name: Option<String>,
    properties: Option<Properties>,
    enabled: Option<bool>,
}

impl_command!(UpdateConnectionCommand, CommandType::UpdateConnection);
into_event!(UpdateConnectionCommand);

#[derive(Debug)]
pub struct UpdateConnectionCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for UpdateConnectionCommandHandler {
    type Command = UpdateConnectionCommand;
    type Output = Connection;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let mut conn = Connection::find(ctx.pool(), ConnectionQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve connection:{e}"))?
            .ok_or(UpdateConnectionError::ConnectionNotFound(cmd.id))?;

        if cmd.enabled.is_none() && cmd.name.is_none() && cmd.properties.is_none() {
            return Ok(conn);
        }

        if let Some(properties) = cmd.properties {
            let connector_plugin = ctx
                .plugins()
                .get(conn.plugin_id())
                .context("Plugin with id not found")?;

            connector_plugin
                .configuration()
                .validate(&properties)
                .map_err(UpdateConnectionError::from)?;

            conn = conn.set_properties(properties);
        }

        if let Some(name) = cmd.name {
            conn = conn.set_name(name);
        }

        if let Some(enabled) = cmd.enabled {
            conn = conn.set_enabled(enabled);
        }

        Ok(conn.update(ctx.tx()).await.tap_err(|e| {
            tracing::error!("Failed to update connection {}: {e}", cmd.id);
        })?)
    }
}
