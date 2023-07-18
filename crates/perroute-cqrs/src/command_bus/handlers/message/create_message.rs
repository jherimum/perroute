use crate::command_bus::{
    bus::CommandBusContext, commands::CreateMessageCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::{
        message::{Message, MessageBuilder, Status},
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
                .expect("SchemasQueryBuilder error"),
        )
        .await
        .expect("error de sql")
        .expect("nao encontrado");

        schema.schema().validate(cmd.payload()).unwrap();

        MessageBuilder::default()
            .id(*cmd.message_id())
            .status(Status::Pending)
            .payload(cmd.payload().clone())
            .scheduled_to(*cmd.scheduled_to())
            .schema_id(*schema.id())
            .message_type_id(*schema.message_type_id())
            .channel_id(*schema.channel_id())
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .map_err(CommandBusError::from)
    }
}
