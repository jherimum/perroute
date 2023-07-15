use std::ops::Add;

use crate::command_bus::{
    bus::CommandBusContext, commands::CreateApiKeyCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use chrono::{Duration, NaiveDateTime, Utc};
use perroute_commons::types::actor::Actor;
use perroute_storage::models::api_key::{ApiKey, ApiKeyBuilder};

#[derive(Debug)]
pub struct CreateApiKeyCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for CreateApiKeyCommandHandler {
    type Command = CreateApiKeyCommand;
    type Output = (ApiKey, String);

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let expires_at = cmd
            .expiration_in_hours()
            .map(|d| d as i64)
            .map(Duration::hours)
            .map(|d| Utc::now().add(d).naive_utc());

        let value = create_api_key_value();

        ApiKeyBuilder::default()
            .id(*cmd.api_key_id())
            .name(cmd.name().clone())
            .expires_at(expires_at)
            .created_at(Utc::now().naive_utc())
            .channel_id(*cmd.channel_id())
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .map_err(CommandBusError::from)
            .map(|api_key| (api_key, value))
    }
}

fn create_api_key_value() -> String {
    let value = uuid::Uuid::new_v4().to_string();
    todo!()
}
