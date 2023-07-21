use crate::command_bus::{
    bus::CommandBusContext, commands::CreateMessageCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::{
        message::{Message, MessageBuilder, Status},
        schema::{Schema, SchemasQueryBuilder},
    },
    query::FetchableModel,
};
use sqlx::{types::Json, PgPool};

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
        let schema = retrieve_schema(ctx.pool(), cmd.schema_id()).await?;

        schema.schema().validate(cmd.payload()).unwrap();

        MessageBuilder::default()
            .id(*cmd.message_id())
            .status(Status::Pending)
            .payload(cmd.payload().clone())
            .scheduled_to(*cmd.scheduled_to())
            .schema_id(*schema.id())
            .message_type_id(*schema.message_type_id())
            .channel_id(*schema.channel_id())
            .include_dispatcher_types(Json(cmd.include_dispatcher_types().clone()))
            .exclude_dispatcher_types(Json(cmd.exclude_dispatcher_types().clone()))
            .recipient(Json(cmd.recipient().clone()))
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .map_err(CommandBusError::from)
    }
}

async fn retrieve_schema(pool: &PgPool, id: &Id) -> Result<Schema, CommandBusError> {
    Ok(Schema::find(
        pool,
        SchemasQueryBuilder::default()
            .id(Some(*id))
            .build()
            .expect("SchemasQueryBuilder error"),
    )
    .await
    .expect("error de sql")
    .expect("nao encontrado"))
}
