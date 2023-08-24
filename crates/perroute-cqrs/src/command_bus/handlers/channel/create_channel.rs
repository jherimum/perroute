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
    priority::Priority,
    properties::{Properties, PropertiesError},
};
use perroute_connectors::types::{dispatch_type::DispatchType, plugin_id::ConnectorPluginId};
use perroute_storage::{
    models::{
        business_unit::{BusinessUnit, BusinessUnitQuery},
        channel::{Channel, ChannelBuilder},
        connection::{Connection, ConnectionQuery},
    },
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Business unit {0} not found")]
    BusinessUnitNotFound(Id),

    #[error("Connection {0} not found")]
    ConnectionNotFound(Id),

    #[error("Plugin {0} not found")]
    PluginNotFound(ConnectorPluginId),

    #[error("Dispatch type  {0} not supoorted by plugin")]
    DispatchTypeNotSupported(DispatchType),

    #[error("Invalid properties")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateChannelCommand {
    id: Id,
    connection_id: Id,
    business_unit_id: Id,
    dispatch_type: DispatchType,
    dispatch_properties: Properties,
    priority: Priority,
}

impl_command!(CreateChannelCommand, CommandType::CreateChannel);
into_event!(CreateChannelCommand);

#[derive(Debug)]
pub struct CreateChannelCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for CreateChannelCommandHandler {
    type Command = CreateChannelCommand;
    type Output = Channel;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let _ = BusinessUnit::find(ctx.pool(), BusinessUnitQuery::with_id(cmd.business_unit_id))
            .await
            .tap_err(|e| {
                tracing::error!(
                    "Failed to retrieve business unit {}: {e}",
                    cmd.business_unit_id
                )
            })?
            .ok_or_else(|| Error::BusinessUnitNotFound(cmd.business_unit_id))?;

        let conn = Connection::find(ctx.pool(), ConnectionQuery::with_id(cmd.connection_id))
            .await
            .tap_err(|e| {
                tracing::error!("Failed to retrieve connection {}: {e}", cmd.connection_id)
            })?
            .ok_or_else(|| Error::ConnectionNotFound(cmd.connection_id))?;

        let plugin = ctx
            .plugins()
            .get(conn.plugin_id())
            .ok_or_else(|| Error::PluginNotFound(*conn.plugin_id()))?;

        let disp = plugin
            .dispatcher(cmd.dispatch_type())
            .ok_or_else(|| Error::DispatchTypeNotSupported(cmd.dispatch_type))?;

        disp.configuration()
            .validate(cmd.dispatch_properties())
            .map_err(Error::from)?;

        Ok(ChannelBuilder::default()
            .id(cmd.id)
            .connection_id(cmd.connection_id)
            .business_unit_id(cmd.business_unit_id)
            .properties(cmd.dispatch_properties)
            .dispatch_type(cmd.dispatch_type)
            .priority(cmd.priority)
            .enabled(false)
            .build()
            .context("Failed to build Channel")?
            .save(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to save channel:{e}"))?)
    }
}
