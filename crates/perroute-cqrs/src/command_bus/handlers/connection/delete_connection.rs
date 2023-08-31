use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::{
        channel::{Channel, ChannelQuery},
        connection::{Connection, ConnectionQueryBuilder},
    },
    query::FetchableModel,
};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum DeleteConnectionError {
    #[error("Connection with id {0} not found")]
    ConnectionNotFound(Id),

    #[error("Connection {0} could not be deleted: {1}")]
    DeleteError(Id, String),
}

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
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let conn = Connection::find(
            ctx.pool(),
            ConnectionQueryBuilder::default()
                .id(Some(cmd.id))
                .build()
                .unwrap(),
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve connection: {e}"))?
        .ok_or(DeleteConnectionError::ConnectionNotFound(cmd.id))?;

        if Channel::exists(ctx.pool(), ChannelQuery::with_connection(*conn.id()))
            .await
            .tap_err(|e| tracing::error!("Faled to check if channel exists: {e}"))?
        {
            return Err(DeleteConnectionError::DeleteError(
                cmd.id,
                "Connection is still in use by channels".to_string(),
            )
            .into());
        }

        Ok(conn
            .delete(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to delete connection: {e}"))
            .map(|_| ())?)
    }
}
