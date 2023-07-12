use crate::command_bus::{
    bus::CommandBusContext, commands::CreateTemplateCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::{
        schema::{Schema, SchemasQueryBuilder},
        template::{Template, TemplateBuilder},
    },
    query::FetchableModel,
};

#[derive(Debug)]
pub struct CreateTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplatelError {}

#[async_trait]
impl CommandHandler for CreateTemplateCommandHandler {
    type Command = CreateTemplateCommand;
    type Output = Template;

    #[tracing::instrument(name = "create_temploate_handler", skip(self, ctx))]
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
        .await?
        .unwrap();

        TemplateBuilder::default()
            .id(*cmd.template_id())
            .name(cmd.name())
            .subject(cmd.subject().clone())
            .text(cmd.text().clone())
            .html(cmd.html().clone())
            .schema_id(*cmd.schema_id())
            .message_type_id(*schema.message_type_id())
            .channel_id(*schema.channel_id())
            .build()
            .unwrap()
            .save(ctx.pool())
            .await
            .map_err(Into::into)
    }
}
