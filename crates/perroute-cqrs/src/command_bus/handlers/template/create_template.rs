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
use perroute_connectors::types::dispatch_type::DispatchType;
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
    id: Id,
    name: String,
    subject: Option<String>,
    html: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>,
    dispatch_type: DispatchType,
    schema_id: Id,
    vars: Vars
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

    #[tracing::instrument(name = "create_template_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let schema = Schema::find(ctx.pool(), SchemasQueryBuilder::default().build().unwrap())
            .await
            .unwrap()
            .unwrap();

        TemplateBuilder::default()
            .id(cmd.id)
            .name(cmd.name)
            .subject(cmd.subject)
            .text(cmd.text)
            .html(cmd.html)
            .active(false)
            .business_unit_id(*schema.business_unit_id())
            .dispatch_type(cmd.dispatch_type)
            .message_type_id(*schema.message_type_id())
            .vars(cmd.vars)
            .build()
            .unwrap()
            .save(ctx.pool())
            .await
            .map_err(Into::into)
    }
}
