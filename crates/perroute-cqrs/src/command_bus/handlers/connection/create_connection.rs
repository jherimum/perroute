use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
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
use perroute_connectors::types::ConnectorPluginId;
use perroute_storage::models::connection::{Connection, ConnectionBuilder};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Plugin with id {0} not found")]
    PluginNotFound(ConnectorPluginId),

    #[error("Invalid properties: {0}")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateConnectionCommand {
    id: Id,
    name: String,
    plugin_id: ConnectorPluginId,
    properties: Properties,
}

impl_command!(CreateConnectionCommand, CommandType::CreateConnection);
into_event!(CreateConnectionCommand);

#[derive(Debug)]
pub struct CreateConnectionCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for CreateConnectionCommandHandler {
    type Command = CreateConnectionCommand;
    type Output = Connection;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let connector_plugin = ctx
            .plugins()
            .get(cmd.plugin_id())
            .ok_or(Error::PluginNotFound(cmd.plugin_id))?;

        connector_plugin
            .configuration()
            .validate(&cmd.properties)
            .map_err(Error::from)?;

        Ok(ConnectionBuilder::default()
            .id(cmd.id)
            .name(cmd.name)
            .plugin_id(cmd.plugin_id)
            .properties(cmd.properties)
            .enabled(false)
            .build()
            .context("Failed to build connection")?
            .save(ctx.tx())
            .await
            .tap_err(|e| {
                tracing::error!("Failed to save connection: {e}");
            })?)
    }
}
