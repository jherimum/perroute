use crate::command_bus::{
    bus::CommandBusContext, commands::CreateTemplateCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use async_trait::async_trait;
use perroute_storage::models::{
    schema::Schema,
    template::{Template, TemplateBuilder},
};

#[derive(Debug)]
pub struct CreateTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplatelError {}

#[async_trait]
impl CommandHandler for CreateTemplateCommandHandler {
    type Command = CreateTemplateCommand;
    type Output = Template;

    #[tracing::instrument(name = "create_channel_command", skip(self))]
    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let schema = Schema::find_by_id(ctx.pool(), *cmd.schema_id())
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
