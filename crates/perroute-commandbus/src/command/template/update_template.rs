use crate::{
    bus::Ctx,
    command::{Command, CommandResult},
};
use async_trait::async_trait;
use perroute_commons::types::{
    actor::Actor, command_type::CommandType, id::Id, template::TemplateSnippet,
};
use perroute_storage::{
    models::template::{Template, TemplatesQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder)]
pub struct UpdateTemplateCommand {
    id: Id,
    name: Option<String>,
    subject: Option<Option<TemplateSnippet>>,
    html: Option<Option<TemplateSnippet>>,
    text: Option<Option<TemplateSnippet>>,
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum UpdateTemplatelError {
    #[error("Template not found: {0}")]
    TemplateNotFound(Id),
}

#[async_trait]
impl Command for UpdateTemplateCommand {
    type Output = Template;

    #[tracing::instrument(name = "update_template_handler", skip(self, ctx))]
    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> CommandResult<Self::Output> {
        let mut actual_template = Template::find(ctx.pool(), TemplatesQuery::with_id(self.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve template:{e}"))?
            .ok_or(UpdateTemplatelError::TemplateNotFound(self.id))?;

        if self.name.is_none() & self.subject.is_none() & self.html.is_none() & self.text.is_none()
        {
            return Ok(actual_template);
        }

        if let Some(name) = self.name.clone() {
            actual_template = actual_template.set_name(name);
        }

        if let Some(subject) = self.subject.clone() {
            actual_template = actual_template.set_subject(subject);
        }

        if let Some(html) = self.html.clone() {
            actual_template = actual_template.set_html(html);
        }

        if let Some(text) = self.text.clone() {
            actual_template = actual_template.set_text(text);
        }

        Ok(actual_template
            .update(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to update template:{e}"))?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::UpdateTemplate
    }

    fn supports(&self, actor: &Actor) -> bool {
        true
    }
}
