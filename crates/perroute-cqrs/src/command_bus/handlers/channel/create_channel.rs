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
use perroute_connectors::{types::DispatchType, Plugins};
use perroute_storage::{
    models::{
        business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
        channel::{Channel, ChannelBuilder, Priority},
        connection::{Connection, ConnectionQueryBuilder},
    },
    query::FetchableModel,
};

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
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let business_unit = BusinessUnit::find(
            ctx.pool(),
            BusinessUnitQueryBuilder::default()
                .id(Some(cmd.business_unit_id))
                .build()
                .unwrap(),
        )
        .await
        .unwrap()
        .unwrap();

        let conn = Connection::find(
            ctx.pool(),
            ConnectionQueryBuilder::default()
                .id(Some(cmd.connection_id))
                .build()
                .unwrap(),
        )
        .await
        .unwrap()
        .unwrap();

        let plugin = conn.plugin(ctx.plugins()).unwrap();
        let disp = plugin.dispatcher(cmd.dispatch_type()).unwrap();
        disp.configuration()
            .validate(cmd.dispatch_properties())
            .unwrap();

        Ok(ChannelBuilder::default()
            .id(cmd.id)
            .connection_id(cmd.connection_id)
            .business_unit_id(cmd.business_unit_id)
            .properties(cmd.dispatch_properties)
            .dispatch_type(cmd.dispatch_type)
            .priority(cmd.priority)
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .unwrap())
    }
}
