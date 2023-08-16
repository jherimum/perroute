use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_connectors::Plugins;
use perroute_storage::{
    models::channel::{Channel, ChannelQueryBuilder},
    query::FetchableModel,
};

#[derive(
    Debug, serde::Serialize, Clone, PartialEq, Eq, derive_builder::Builder, derive_getters::Getters,
)]
pub struct DeleteChannelCommand {
    id: Id,
}
impl_command!(DeleteChannelCommand, CommandType::DeleteChannel);
into_event!(DeleteChannelCommand);

#[derive(Debug)]
pub struct DeleteChannelCommandHandler {
    pub plugins: Plugins,
}

#[async_trait::async_trait]
impl CommandHandler for DeleteChannelCommandHandler {
    type Command = DeleteChannelCommand;
    type Output = bool;

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

        Ok(channel.delete(ctx.tx()).await.unwrap())
    }
}
