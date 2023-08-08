use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use async_trait::async_trait;
use perroute_commons::types::{
    actor::Actor, dispatch_type::DispatchType, id::Id, template::TemplateSnippet,
};
use perroute_storage::{
    models::{
        schema::{Schema, SchemasQueryBuilder},
        template::{Template, TemplateBuilder},
    },
    query::FetchableModel,
};

command!(
    CreateTemplateCommand,
    CommandType::CreateTemplate,
    template_id: Id,
    schema_id: Id,
    channel_id: Id,
    name: String,
    html: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>,
    subject: Option<TemplateSnippet>,
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
        let schema = Schema::find(
            ctx.pool(),
            SchemasQueryBuilder::default()
                .id(Some(*cmd.schema_id()))
                .build()
                .unwrap(),
        )
        .await?
        .unwrap();

        TemplateBuilder::default()
            .id(cmd.template_id)
            .name(cmd.name)
            .subject(cmd.subject)
            .text(cmd.text)
            .html(cmd.html)
            .schema_id(cmd.schema_id)
            .message_type_id(*schema.message_type_id())
            .channel_id(cmd.channel_id)
            .dispatch_type(cmd.dispatch_type)
            .build()
            .unwrap()
            .save(ctx.pool())
            .await
            .map_err(Into::into)
    }
}
