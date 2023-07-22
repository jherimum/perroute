use std::any;

use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::message::{Message, MessageQueryBuilder},
    query::FetchableModel,
};
use tap::TapFallible;

use crate::command_bus::{
    bus::CommandBusContext, commands::DistributeMessageCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct DistributeMessageCommandHandler;

#[async_trait]
impl CommandHandler for DistributeMessageCommandHandler {
    type Command = DistributeMessageCommand;

    type Output = Message;

    #[tracing::instrument(name = "distribute_message_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let message = Message::find(
            ctx.pool(),
            MessageQueryBuilder::default()
                .build()
                .tap_err(|e| tracing::error!("Failed to build MessageQueryBuilder: {e}"))
                .map_err(anyhow::Error::from)?,
        )
        .await?;

        todo!()
    }
}
