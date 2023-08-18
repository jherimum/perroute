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
use perroute_storage::{
    models::connection::{Connection, ConnectionQueryBuilder},
    query::FetchableModel,
};
use serde::Serialize;

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
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let conn = Connection::find(
            ctx.pool(),
            ConnectionQueryBuilder::default()
                .id(Some(cmd.id))
                .build()
                .unwrap(),
        )
        .await
        .unwrap()
        .unwrap();

        let connector_plugin = ctx.plugins().get(conn.plugin_id()).unwrap();
        connector_plugin
            .configuration()
            .validate(&cmd.properties)
            .unwrap();

        Ok(conn
            .set_enabled(cmd.enabled)
            .set_name(cmd.name)
            .set_properties(cmd.properties)
            .update(ctx.tx())
            .await
            .unwrap())
    }
}
