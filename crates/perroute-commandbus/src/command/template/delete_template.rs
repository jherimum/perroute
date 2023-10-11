use crate::{
    bus::Ctx,
    command::{Command, CommandResult},
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, command_type::CommandType, id::Id};
use perroute_storage::{
    models::template::{Template, TemplatesQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder)]
pub struct DeleteTemplateCommand {
    id: Id,
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum DeleteTemplateError {
    #[error("Template not found: {0}")]
    TemplateNotFound(Id),
}

#[async_trait]
impl Command for DeleteTemplateCommand {
    type Output = bool;

    #[tracing::instrument(name = "delete_template_handler", skip(self, ctx))]
    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> CommandResult<Self::Output> {
        let template = Template::find(ctx.pool(), TemplatesQuery::with_id(self.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve template:{e}"))?
            .ok_or(DeleteTemplateError::TemplateNotFound(self.id))?;

        Ok(template
            .clone()
            .delete(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to delete template:{e}"))?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::DeleteTemplate
    }

    fn supports(&self, actor: &Actor) -> bool {
        true
    }
}
