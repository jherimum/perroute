use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, id::Id, properties::Properties};
use perroute_connectors::{types::ConnectorPluginId, Plugins};
use perroute_storage::models::connection::{Connection, ConnectionBuilder};
use serde::Serialize;
use sqlx::types::Json;

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
    pub plugins: Plugins,
}

#[async_trait::async_trait]
impl CommandHandler for CreateConnectionCommandHandler {
    type Command = CreateConnectionCommand;
    type Output = Connection;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let connector_plugin = self.plugins.get(cmd.plugin_id).unwrap();
        connector_plugin
            .configuration()
            .validate(&cmd.properties)
            .unwrap();

        Ok(ConnectionBuilder::default()
            .id(cmd.id)
            .name(cmd.name)
            .plugin_id(cmd.plugin_id)
            .properties(Json(cmd.properties))
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .unwrap())
    }
}
