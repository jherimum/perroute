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
use perroute_storage::{
    models::connection::{Connection, ConnectionQuery},
    query::FetchableModel,
};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Connection with id {0} not found")]
    ConnectionNotFound(Id),

    #[error("Invalid properties: {0}")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct UpdateConnectionCommand {
    id: Id,
    name: String,
    properties: Properties,
    enabled: bool,
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
    ) -> Result<Self::Output, CommandBusError> {
        let conn = Connection::find(ctx.pool(), ConnectionQuery::with_id(cmd.id))
            .await?
            .ok_or(Error::ConnectionNotFound(cmd.id))?;

        let connector_plugin = ctx
            .plugins()
            .get(conn.plugin_id())
            .context("Plugin with id not found")?;

        connector_plugin
            .configuration()
            .validate(&cmd.properties)
            .map_err(Error::from)?;

        Ok(conn
            .set_enabled(cmd.enabled)
            .set_name(cmd.name)
            .set_properties(cmd.properties)
            .update(ctx.tx())
            .await
            .tap_err(|e| {
                tracing::error!("Failed to update connection {}: {e}", cmd.id);
            })?)
    }
}
