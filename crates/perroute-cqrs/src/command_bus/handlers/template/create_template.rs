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
use perroute_connectors::types::DispatchType;
use perroute_storage::models::template::{Template, TemplateBuilder};

command!(
    CreateTemplateCommand,
    CommandType::CreateTemplate,
    template_id: Id,
    bu_id: Id,
    message_type_id: Id,
    name: String,
    subject: Option<String>,
    html: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>,
    dispatch_type: DispatchType
);
into_event!(CreateTemplateCommand);

#[derive(Debug)]
pub struct CreateTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplatelError {}

#[async_trait]
impl CommandHandler for CreateTemplateCommandHandler {
    type Command = CreateTemplateCommand;
    type Output = Template;

    #[tracing::instrument(name = "create_temploate_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        TemplateBuilder::default()
            .id(cmd.template_id)
            .subject(cmd.subject)
            .text(cmd.text)
            .html(cmd.html)
            .bu_id(cmd.bu_id)
            .dispatch_type(cmd.dispatch_type)
            .message_type_id(cmd.message_type_id)
            .build()
            .unwrap()
            .save(ctx.pool())
            .await
            .map_err(Into::into)
    }
}
