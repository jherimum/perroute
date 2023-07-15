use crate::command_bus::{
    bus::CommandBusContext, commands::RevokeApiKeyCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use chrono::Utc;
use perroute_commons::types::actor::Actor;
use perroute_storage::{
    models::api_key::{ApiKey, ApiKeyQueryBuilder},
    query::FetchableModel,
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
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let mut api_key = ApiKey::find(
            ctx.pool(),
            ApiKeyQueryBuilder::default()
                .id(Some(*cmd.api_key_id()))
                .build()
                .unwrap(),
        )
        .await?
        .unwrap();

        if !api_key.revoked() {
            api_key = api_key
                .set_revoked_at(Utc::now().naive_utc())
                .update(ctx.tx())
                .await?;
        }

        Ok(api_key)
    }
}
