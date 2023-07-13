use crate::command_bus::{
    bus::CommandBusContext, commands::CreateApiKeyCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::types::actor::Actor;
use perroute_storage::models::api_key::ApiKey;

#[derive(Debug)]
pub struct CreateApiKeyCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for CreateApiKeyCommandHandler {
    type Command = CreateApiKeyCommand;
    type Output = ApiKey;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        todo!()
    }
}
