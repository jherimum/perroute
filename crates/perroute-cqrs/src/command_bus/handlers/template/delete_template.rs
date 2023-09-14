use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    into_event,
};
use async_trait::async_trait;
use derive_getters::Getters;
use perroute_commons::types::id::Id;
use perroute_storage::{
    models::template::{Template, TemplatesQuery},
    query::FetchableModel,
};
use sqlx::PgPool;
use tap::TapFallible;

command!(
    DeleteTemplateCommand,
    CommandType::DeleteTemplate,
    id: Id
);
into_event!(DeleteTemplateCommand);

#[derive(Debug, Getters)]
pub struct DeleteTemplateCommandHandler {
    pool: PgPool,
}

impl DeleteTemplateCommandHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

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
        ctx: &mut CommandBusContext,
        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let template = Template::find(ctx.pool(), TemplatesQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve template:{e}"))?
            .ok_or(DeleteTemplateError::TemplateNotFound(cmd.id))?;

        Ok(template
            .clone()
            .delete(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to delete template:{e}"))?)
    }
}
