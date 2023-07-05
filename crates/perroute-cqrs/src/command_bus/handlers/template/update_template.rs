use async_trait::async_trait;
use perroute_storage::models::template::Template;

use crate::command_bus::{
    bus::CommandBusContext, commands::UpdateTemplateCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct UpdateTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplatelError {}

#[async_trait]
impl CommandHandler for UpdateTemplateCommandHandler {
    type Command = UpdateTemplateCommand;
    type Output = Template;

    #[tracing::instrument]
    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        _: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        todo!()
    }
}
