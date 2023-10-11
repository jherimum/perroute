use crate::{bus::Ctx, command::Command, error::CommandBusError};
use anyhow::Context;
use perroute_commons::types::{
    actor::Actor,
    command_type::CommandType,
    id::Id,
    properties::{Properties, PropertiesError},
};
use perroute_connectors::types::plugin_id::ConnectorPluginId;
use perroute_storage::models::connection::{Connection, ConnectionBuilder};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateConnectionError {
    #[error("Plugin with id {0} not found")]
    PluginNotFound(ConnectorPluginId),

    #[error("Invalid properties: {0}")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug, derive_builder::Builder)]
pub struct CreateConnectionCommand {
    id: Id,
    name: String,
    plugin_id: ConnectorPluginId,
    properties: Properties,
}

#[async_trait::async_trait]
impl Command for CreateConnectionCommand {
    type Output = Connection;

    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        let connector_plugin = ctx
            .plugins()
            .get(&self.plugin_id)
            .ok_or(CreateConnectionError::PluginNotFound(self.plugin_id))?;

        connector_plugin
            .configuration()
            .validate(&self.properties)
            .map_err(CreateConnectionError::from)?;

        Ok(ConnectionBuilder::default()
            .id(self.id)
            .name(self.name.clone())
            .plugin_id(self.plugin_id)
            .properties(self.properties.clone())
            .enabled(false)
            .build()
            .context("Failed to build connection")?
            .save(ctx.pool())
            .await
            .tap_err(|e| {
                tracing::error!("Failed to save connection: {e}");
            })?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::CreateConnection
    }

    fn supports(&self, _actor: &Actor) -> bool {
        true
    }
}
