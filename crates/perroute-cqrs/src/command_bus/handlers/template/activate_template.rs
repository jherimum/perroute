use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::template::{Template, TemplatesQueryBuilder},
    query::FetchableModel,
};

command!(
    ActivateTemplateCommand,
    CommandType::ActivateTemplate,
    template_id: Id
);
into_event!(ActivateTemplateCommand);

#[derive(Debug)]
pub struct ActivateTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum ActivateTemplateError {}

#[async_trait]
impl CommandHandler for ActivateTemplateCommandHandler {
    type Command = ActivateTemplateCommand;
    type Output = Template;

    #[tracing::instrument(name = "activate_template_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let actual_template = Template::find(
            ctx.pool(),
            TemplatesQueryBuilder::default()
                .id(Some(*cmd.template_id()))
                .build()
                .unwrap(),
        )
        .await?
        .unwrap();

        for template in Template::query(
            ctx.pool(),
            TemplatesQueryBuilder::default()
                .dispatch_type(Some(*actual_template.dispatch_type()))
                .schema_id(Some(*actual_template.schema_id()))
                .build()
                .unwrap(),
        )
        .await?
        {
            template.set_active(false).update(ctx.tx()).await?;
        }

        let template = actual_template.set_active(true).update(ctx.tx()).await?;

        Ok(template)
    }
}
