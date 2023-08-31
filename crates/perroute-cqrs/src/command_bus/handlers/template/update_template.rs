use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    into_event,
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id, template::TemplateSnippet, vars::Vars};
use perroute_storage::{
    models::template::{Template, TemplatesQuery},
    query::FetchableModel,
};
use tap::TapFallible;

command!(
    UpdateTemplateCommand,
    CommandType::UpdateTemplate,
    id: Id,
    name: Option<String>,
    subject: Option<Option<TemplateSnippet>>,
    html: Option<Option<TemplateSnippet>>,
    text: Option<Option<TemplateSnippet>>,
    vars: Option<Vars>,
    active: Option<bool>
);
into_event!(UpdateTemplateCommand);

#[derive(Debug)]
pub struct UpdateTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum UpdateTemplatelError {
    #[error("Template not found: {0}")]
    TemplateNotFound(Id),
}

#[async_trait]
impl CommandHandler for UpdateTemplateCommandHandler {
    type Command = UpdateTemplateCommand;
    type Output = Template;

    #[tracing::instrument(name = "update_template_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let mut actual_template = Template::find(ctx.pool(), TemplatesQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve template:{e}"))?
            .ok_or(UpdateTemplatelError::TemplateNotFound(cmd.id))?;

        if cmd.name.is_none()
            & cmd.subject.is_none()
            & cmd.html.is_none()
            & cmd.text.is_none()
            & cmd.vars.is_none()
            & cmd.active.is_none()
        {
            return Ok(actual_template);
        }

        if let Some(active) = cmd.active {
            if active {
                Template::inactivate_all(
                    ctx.tx(),
                    *actual_template.schema_id(),
                    *actual_template.dispatch_type(),
                )
                .await
                .tap_err(|e| tracing::error!("Failed to inactivate all templates:{e}"))?;
            }

            actual_template = actual_template.set_active(active);
        }

        if let Some(name) = cmd.name {
            actual_template = actual_template.set_name(name);
        }

        if let Some(vars) = cmd.vars {
            actual_template = actual_template.set_vars(vars);
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
            .update(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to update template:{e}"))?)
    }
}
