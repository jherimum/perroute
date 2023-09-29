use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    into_event,
};
use async_trait::async_trait;
use derive_getters::Getters;
use perroute_commons::types::{id::Id, template::TemplateSnippet};
use perroute_storage::{
    models::template::{Template, TemplatesQuery},
    query::FetchableModel,
};
use sqlx::PgPool;
use tap::TapFallible;

command!(
    UpdateTemplateCommand,
    CommandType::UpdateTemplate,
    id: Id,
    name: Option<String>,
    subject: Option<Option<TemplateSnippet>>,
    html: Option<Option<TemplateSnippet>>,
    text: Option<Option<TemplateSnippet>>

);
into_event!(UpdateTemplateCommand);

#[derive(thiserror::Error, Debug, Clone)]
pub enum UpdateTemplatelError {
    #[error("Template not found: {0}")]
    TemplateNotFound(Id),
}

#[derive(Debug, Getters)]
pub struct UpdateTemplateCommandHandler {
    pool: PgPool,
}

impl UpdateTemplateCommandHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommandHandler for UpdateTemplateCommandHandler {
    type Command = UpdateTemplateCommand;
    type Output = Template;

    #[tracing::instrument(name = "update_template_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext,

        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let mut actual_template = Template::find(ctx.pool(), TemplatesQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve template:{e}"))?
            .ok_or(UpdateTemplatelError::TemplateNotFound(cmd.id))?;

        if cmd.name.is_none() & cmd.subject.is_none() & cmd.html.is_none() & cmd.text.is_none() {
            return Ok(actual_template);
        }

        if let Some(name) = cmd.name {
            actual_template = actual_template.set_name(name);
        }

        if let Some(subject) = cmd.subject {
            actual_template = actual_template.set_subject(subject);
        }

        if let Some(html) = cmd.html {
            actual_template = actual_template.set_html(html);
        }

        if let Some(text) = cmd.text {
            actual_template = actual_template.set_text(text);
        }

        Ok(actual_template
            .update(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to update template:{e}"))?)
    }
}
