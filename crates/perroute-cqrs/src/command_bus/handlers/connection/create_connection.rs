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
use perroute_connectors::{types::plugin_id::ConnectorPluginId, Plugins};
use perroute_storage::models::connection::{Connection, ConnectionBuilder};
use serde::Serialize;
use sqlx::PgPool;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateConnectionError {
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
pub struct CreateConnectionCommandHandler {
    pool: PgPool,
    plugins: Plugins,
}

impl CreateConnectionCommandHandler {
    pub fn new(pool: PgPool, plugins: Plugins) -> Self {
        Self { pool, plugins }
    }
}

#[async_trait::async_trait]
impl CommandHandler for CreateConnectionCommandHandler {
    type Command = CreateConnectionCommand;
    type Output = Connection;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let connector_plugin = ctx
            .plugins()
            .get(cmd.plugin_id())
            .ok_or(CreateConnectionError::PluginNotFound(cmd.plugin_id))?;

        connector_plugin
            .configuration()
            .validate(&cmd.properties)
            .map_err(CreateConnectionError::from)?;

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
