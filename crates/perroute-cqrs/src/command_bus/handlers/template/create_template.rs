use async_trait::async_trait;
use perroute_storage::models::template::Template;

use crate::command_bus::{
    bus::CommandBusContext, commands::CreateTemplateCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct CreateTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplatelError {}

#[async_trait]
impl CommandHandler for CreateTemplateCommandHandler {
    type Command = CreateTemplateCommand;
    type Output = Template;

    #[tracing::instrument(name = "create_channel_command", skip(self))]
    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        _: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        todo!()
    }
}
