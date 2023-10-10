use crate::{bus::Ctx, command::Command, error::CommandBusError};
use anyhow::Context;
use perroute_commons::types::{
    actor::Actor,
    command_type::CommandType,
    id::Id,
    properties::{Properties, PropertiesError},
};
use perroute_storage::{
    models::connection::{Connection, ConnectionQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum UpdateConnectionError {
    #[error("Connection with id {0} not found")]
    ConnectionNotFound(Id),

    #[error("Invalid properties: {0}")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug)]
pub struct UpdateConnectionCommand {
    id: Id,
    name: Option<String>,
    properties: Option<Properties>,
    enabled: Option<bool>,
}

#[async_trait::async_trait]
impl Command for UpdateConnectionCommand {
    type Output = Connection;

    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        let mut conn = Connection::find(ctx.pool(), ConnectionQuery::with_id(self.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve connection:{e}"))?
            .ok_or(UpdateConnectionError::ConnectionNotFound(self.id))?;

        if self.enabled.is_none() && self.name.is_none() && self.properties.is_none() {
            return Ok(conn);
        }

        if let Some(properties) = self.properties.clone() {
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

        if let Some(name) = self.name.clone() {
            conn = conn.set_name(name);
        }

        if let Some(enabled) = self.enabled {
            conn = conn.set_enabled(enabled);
        }

        Ok(conn.update(ctx.pool()).await.tap_err(|e| {
            tracing::error!("Failed to update connection {}: {e}", self.id);
        })?)
    }

    fn supports(&self, _actor: &Actor) -> bool {
        true
    }

    fn command_type(&self) -> CommandType {
        CommandType::UpdateConnection
    }
}
