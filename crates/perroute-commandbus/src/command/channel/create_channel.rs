use crate::{bus::Ctx, command::Command, error::CommandBusError};
use anyhow::Context;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor,
    command_type::CommandType,
    id::Id,
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
pub enum CreateChannelError {
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

#[derive(Debug, derive_builder::Builder)]
pub struct CreateChannelCommand {
    id: Id,
    connection_id: Id,
    business_unit_id: Id,
    dispatch_type: DispatchType,
    dispatch_properties: Properties,
}

#[async_trait::async_trait]
impl Command for CreateChannelCommand {
    type Output = Channel;

    async fn handle<'ctx>(&self, ctx: &mut Ctx<'ctx>) -> Result<Channel, CommandBusError> {
        let _ = BusinessUnit::find(
            ctx.pool(),
            BusinessUnitQuery::with_id(self.business_unit_id),
        )
        .await
        .tap_err(|e| {
            tracing::error!(
                "Failed to retrieve business unit {}: {e}",
                self.business_unit_id
            )
        })?
        .ok_or_else(|| CreateChannelError::BusinessUnitNotFound(self.business_unit_id))?;

        let conn = Connection::find(ctx.pool(), ConnectionQuery::with_id(self.connection_id))
            .await
            .tap_err(|e| {
                tracing::error!("Failed to retrieve connection {}: {e}", self.connection_id)
            })?
            .ok_or_else(|| CreateChannelError::ConnectionNotFound(self.connection_id))?;

        let plugin = ctx
            .plugins()
            .get(conn.plugin_id())
            .ok_or_else(|| CreateChannelError::PluginNotFound(*conn.plugin_id()))?;

        let disp = plugin
            .dispatcher(&self.dispatch_type)
            .ok_or_else(|| CreateChannelError::DispatchTypeNotSupported(self.dispatch_type))?;

        disp.configuration()
            .validate(&self.dispatch_properties)
            .map_err(CreateChannelError::from)?;

        Ok(ChannelBuilder::default()
            .id(self.id)
            .connection_id(self.connection_id)
            .business_unit_id(self.business_unit_id)
            .properties(self.dispatch_properties.clone())
            .dispatch_type(self.dispatch_type)
            .enabled(false)
            .build()
            .context("Failed to build Channel")?
            .save(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to save channel:{e}"))?)
    }

    fn supports(&self, actor: &Actor) -> bool {
        true
    }

    fn command_type(&self) -> CommandType {
        CommandType::CreateChannel
    }
}
