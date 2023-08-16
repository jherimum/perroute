use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_connectors::Plugins;
use perroute_storage::{
    models::connection::{Connection, ConnectionQueryBuilder},
    query::FetchableModel,
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct DeleteConnectionCommand {
    id: Id,
}

impl_command!(DeleteConnectionCommand, CommandType::DeleteConnection);
into_event!(DeleteConnectionCommand);

#[derive(Debug)]
pub struct DeleteConnectionCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for DeleteConnectionCommandHandler {
    type Command = DeleteConnectionCommand;
    type Output = ();

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

        conn.delete(ctx.tx()).await.unwrap();
        Ok(())
    }
}
