use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id, template::TemplateSnippet};
use perroute_storage::{
    models::template::{Template, TemplatesQueryBuilder},
    query::FetchableModel,
};

command!(
    UpdateTemplateCommand,
    CommandType::UpdateTemplate,
    template_id: Id,
    subject: Option<String>,
    html: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>
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

    #[tracing::instrument(name = "update_temploate_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        Template::find(
            ctx.pool(),
            TemplatesQueryBuilder::default()
                .id(Some(*cmd.template_id()))
                .build()
                .unwrap(),
        )
        .await?
        .unwrap()
        .set_html(cmd.html)
        .set_text(cmd.text)
        .set_subject(cmd.subject)
        .save(ctx.tx())
        .await
        .map_err(Into::into)
    }
}
