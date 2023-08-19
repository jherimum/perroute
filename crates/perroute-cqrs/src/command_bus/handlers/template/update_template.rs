use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id, template::TemplateSnippet, vars::Vars};
use perroute_storage::{
    models::template::{Template, TemplatesQuery},
    query::FetchableModel,
};

command!(
    UpdateTemplateCommand,
    CommandType::UpdateTemplate,
    id: Id,
    name: Option<String>,
    subject: Option<Option<String>>,
    html: Option<Option<TemplateSnippet>>,
    text: Option<Option<TemplateSnippet>>,
    vars: Option<Vars>
);
into_event!(UpdateTemplateCommand);

#[derive(Debug)]
pub struct UpdateTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplatelError {}

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
    ) -> Result<Self::Output, CommandBusError> {
        let mut template = Template::find(ctx.pool(), TemplatesQuery::with_id(cmd.id))
            .await?
            .unwrap();

        if cmd.name.is_none()
            & cmd.subject.is_none()
            & cmd.html.is_none()
            & cmd.text.is_none()
            & cmd.vars.is_none()
        {
            return Ok(template);
        }

        if let Some(name) = cmd.name {
            template = template.set_name(name);
        }

        if let Some(vars) = cmd.vars {
            template = template.set_vars(vars);
        }

        if let Some(subject) = cmd.subject {
            template = template.set_subject(subject);
        }

        if let Some(html) = cmd.html {
            template = template.set_html(html);
        }

        if let Some(text) = cmd.text {
            template = template.set_text(text);
        }

        Ok(template.save(ctx.tx()).await?)
    }
}
