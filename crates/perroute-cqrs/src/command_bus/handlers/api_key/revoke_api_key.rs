use perroute_commons::types::actor::Actor;
use perroute_storage::models::api_key::ApiKey;

use crate::command_bus::{
    bus::CommandBusContext, commands::RevokeApiKeyCommand, error::CommandBusError,
    handlers::CommandHandler,
};

#[derive(Debug)]
pub struct RevokeApiKeyCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for RevokeApiKeyCommandHandler {
    type Command = RevokeApiKeyCommand;
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
