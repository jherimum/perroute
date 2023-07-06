use crate::command_bus::{
    bus::CommandBusContext, commands::UpdateTemplateCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use async_trait::async_trait;
use perroute_storage::models::template::Template;

#[derive(Debug)]
pub struct UpdateTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplatelError {}

#[async_trait]
impl CommandHandler for UpdateTemplateCommandHandler {
    type Command = UpdateTemplateCommand;
    type Output = Template;

    #[tracing::instrument]
    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        Template::find_by_id(ctx.pool(), *cmd.template_id())
            .await?
            .unwrap()
            .set_html(cmd.html().clone())
            .set_text(cmd.text().clone())
            .set_subject(cmd.subject().clone())
            .save(ctx.tx())
            .await
            .map_err(Into::into)
    }
}
