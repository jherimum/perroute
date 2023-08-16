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
    DeleteTemplateCommand,
    CommandType::DeleteTemplate,
    template_id: Id
);
into_event!(DeleteTemplateCommand);

#[derive(Debug)]
pub struct DeleteTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplatelError {}

#[async_trait]
impl CommandHandler for DeleteTemplateCommandHandler {
    type Command = DeleteTemplateCommand;
    type Output = bool;

    #[tracing::instrument(name = "delete_template_handler", skip(self, ctx))]
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
        .delete(ctx.tx())
        .await
        .map_err(Into::into)
    }
}
