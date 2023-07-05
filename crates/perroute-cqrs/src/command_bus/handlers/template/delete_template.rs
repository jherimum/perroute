use crate::command_bus::{
    bus::CommandBusContext, commands::DeleteTemplateCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use async_trait::async_trait;

#[derive(Debug)]
pub struct DeleteTemplateCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplatelError {}

#[async_trait]
impl CommandHandler for DeleteTemplateCommandHandler {
    type Command = DeleteTemplateCommand;
    type Output = bool;

    #[tracing::instrument]
    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        _: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        todo!()
    }
}
