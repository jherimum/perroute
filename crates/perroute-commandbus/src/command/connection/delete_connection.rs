use crate::{bus::Ctx, command::Command, error::CommandBusError};
use perroute_commons::types::{actor::Actor, command_type::CommandType, id::Id};
use perroute_storage::{
    models::{
        channel::{Channel, ChannelQuery},
        connection::{Connection, ConnectionQueryBuilder},
    },
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum DeleteConnectionError {
    #[error("Connection with id {0} not found")]
    ConnectionNotFound(Id),

    #[error("Connection {0} could not be deleted: {1}")]
    DeleteError(Id, String),
}

#[derive(Debug, derive_builder::Builder)]
pub struct DeleteConnectionCommand {
    id: Id,
}

#[async_trait::async_trait]
impl Command for DeleteConnectionCommand {
    type Output = ();

    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        let conn = Connection::find(
            ctx.pool(),
            ConnectionQueryBuilder::default()
                .id(Some(self.id))
                .build()
                .unwrap(),
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve connection: {e}"))?
        .ok_or(DeleteConnectionError::ConnectionNotFound(self.id))?;

        if Channel::exists(ctx.pool(), ChannelQuery::with_connection(*conn.id()))
            .await
            .tap_err(|e| tracing::error!("Faled to check if channel exists: {e}"))?
        {
            return Err(DeleteConnectionError::DeleteError(
                self.id,
                "Connection is still in use by channels".to_string(),
            )
            .into());
        }

        Ok(conn
            .delete(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to delete connection: {e}"))
            .map(|_| ())?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::DeleteConnection
    }

    fn supports(&self, actor: &Actor) -> bool {
        true
    }
}
