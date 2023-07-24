use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::{
        message::{Message, MessageQueryBuilder},
        message_dispatch::{MessageDispatchBuilder, MessageDispatchStatus},
        route::{Route, RouteQueryBuilder},
    },
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
        .await?
        .unwrap();

        for route in Route::query(
            ctx.pool(),
            RouteQueryBuilder::default()
                .shema_id(Some(*message.schema_id()))
                .enabled(Some(true))
                .build()
                .unwrap(),
        )
        .await?
        {
            let m = MessageDispatchBuilder::default()
                .id(Id::new())
                .message_id(*message.id())
                .route_id(*route.id())
                .status(MessageDispatchStatus::Pending)
                .build()
                .unwrap()
                .save(ctx.tx())
                .await?;
        }

        todo!()
    }
}
