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
    models::template::{Template, TemplatesQueryBuilder},
    query::FetchableModel,
};
use sqlx::types::Json;

command!(
    UpdateTemplateCommand,
    CommandType::UpdateTemplate,
    id: Id,
    subject: Option<String>,
    html: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>,
    vars: Vars
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
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        Template::find(
            ctx.pool(),
            TemplatesQueryBuilder::default()
                .id(Some(cmd.id))
                .build()
                .unwrap(),
        )
        .await?
        .unwrap()
        .set_html(cmd.html)
        .set_text(cmd.text)
        .set_subject(cmd.subject)
        .set_vars(Json(cmd.vars))
        .save(ctx.tx())
        .await
        .map_err(Into::into)
    }
}
