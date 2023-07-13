use crate::command_bus::{
    bus::CommandBusContext, commands::CreateMessageCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::{
        message::{Message, MessageBuilder},
        schema::{Schema, SchemasQueryBuilder},
    },
    query::FetchableModel,
};

#[derive(Debug)]
pub struct CreateMessageCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for CreateMessageCommandHandler {
    type Command = CreateMessageCommand;
    type Output = Message;

    #[tracing::instrument(name = "create_channel_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let schema = Schema::find(
            ctx.pool(),
            SchemasQueryBuilder::default()
                .id(Some(*cmd.schema_id()))
                .build()
                .unwrap(),
        )
        .await
        .unwrap()
        .unwrap();

        schema.schema().validate(cmd.payload()).unwrap();

        MessageBuilder::default()
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .map_err(CommandBusError::from)
    }
}
