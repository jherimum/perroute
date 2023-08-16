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
use perroute_connectors::Plugins;
use perroute_storage::{
    models::channel::{Channel, ChannelQueryBuilder},
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::types::Json;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct UpdateChannelCommand {
    id: Id,
    dispatch_properties: Properties,
    enabled: bool,
    priority: i32,
}
impl_command!(UpdateChannelCommand, CommandType::UpdateChannel);
into_event!(UpdateChannelCommand);

#[derive(Debug)]
pub struct UpdateChannelCommandHandler {
    pub plugins: Plugins,
}

#[async_trait::async_trait]
impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;
    type Output = Channel;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let channel = Channel::find(
            ctx.pool(),
            ChannelQueryBuilder::default()
                .id(Some(cmd.id))
                .build()
                .unwrap(),
        )
        .await
        .unwrap()
        .unwrap();

        let conn = channel.connection(ctx.pool()).await.unwrap();
        let plugin = conn.plugin(&self.plugins).unwrap();
        let disp = plugin.dispatcher(channel.dispatch_type()).unwrap();
        disp.configuration()
            .validate(cmd.dispatch_properties())
            .unwrap();

        Ok(channel
            .set_enabled(cmd.enabled)
            .set_priority(cmd.priority)
            .set_properties(Json(cmd.dispatch_properties))
            .update(ctx.tx())
            .await
            .unwrap())
    }
}
