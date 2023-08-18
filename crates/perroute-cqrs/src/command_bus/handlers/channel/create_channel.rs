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
use perroute_connectors::types::{ConnectorPluginId, DispatchType};
use perroute_storage::{
    models::{
        business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
        channel::{Channel, ChannelBuilder},
        connection::{Connection, ConnectionQueryBuilder},
    },
    query::FetchableModel,
};
use tap::TapFallible;

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

#[derive(Debug, thiserror::Error)]
pub enum CreateChannelCommandHandlerError {
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
        let _ = BusinessUnit::find(
            ctx.pool(),
            BusinessUnitQueryBuilder::default()
                .id(Some(cmd.business_unit_id))
                .build()
                .context("Failed to build BusinessUnitQuery")?,
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve business unit: {e}"))?
        .ok_or_else(|| {
            CreateChannelCommandHandlerError::BusinessUnitNotFound(cmd.business_unit_id)
        })?;

        let conn = Connection::find(
            ctx.pool(),
            ConnectionQueryBuilder::default()
                .id(Some(cmd.connection_id))
                .build()
                .context("Failed to build ConnectionQuery")?,
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve connection: {e}"))?
        .ok_or_else(|| CreateChannelCommandHandlerError::ConnectionNotFound(cmd.connection_id))?;

        let plugin = conn
            .plugin(ctx.plugins())
            .ok_or_else(|| CreateChannelCommandHandlerError::PluginNotFound(*conn.plugin_id()))?;

        let disp = plugin.dispatcher(cmd.dispatch_type()).ok_or_else(|| {
            CreateChannelCommandHandlerError::DispatchTypeNotSupported(cmd.dispatch_type)
        })?;

        disp.configuration()
            .validate(cmd.dispatch_properties())
            .map_err(CreateChannelCommandHandlerError::from)?;

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
