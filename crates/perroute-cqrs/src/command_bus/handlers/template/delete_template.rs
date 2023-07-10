use crate::command_bus::{
    bus::CommandBusContext, commands::DeleteTemplateCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use async_trait::async_trait;
use perroute_storage::{models::template::Template, query::FetchableModel};

#[derive(Debug)]
pub struct DeleteTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplatelError {}

#[async_trait]
impl CommandHandler for DeleteTemplateCommandHandler {
    type Command = DeleteTemplateCommand;
    type Output = bool;

    #[tracing::instrument]
    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        Template::find(ctx.pool(), *cmd.template_id())
            .await?
            .unwrap()
            .delete(ctx.tx())
            .await
            .map_err(Into::into)
    }
}
