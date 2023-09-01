use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    into_event,
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::template::{Template, TemplatesQuery},
    query::FetchableModel,
};
use tap::TapFallible;

command!(
    DeleteTemplateCommand,
    CommandType::DeleteTemplate,
    id: Id
);
into_event!(DeleteTemplateCommand);

#[derive(Debug)]
pub struct DeleteTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum DeleteTemplateError {
    #[error("Template not found: {0}")]
    TemplateNotFound(Id),
}

#[async_trait]
impl CommandHandler for DeleteTemplateCommandHandler {
    type Command = DeleteTemplateCommand;
    type Output = bool;

    #[tracing::instrument(name = "delete_template_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let template = Template::find(ctx.pool(), TemplatesQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve template:{e}"))?
            .ok_or(DeleteTemplateError::TemplateNotFound(cmd.id))?;

        Ok(template
            .clone()
            .delete(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to delete template:{e}"))?)
    }
}
