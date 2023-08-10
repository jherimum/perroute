use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use async_trait::async_trait;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, code::Code, id::Id, vars::Vars};
use perroute_messaging::events::EventType;
use perroute_storage::{
    models::channel::{Channel, ChannelBuilder, ChannelsQueryBuilder},
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::{types::Json, PgPool};
use tap::TapFallible;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateChannelCommand {
    #[builder(default)]
    channel_id: Id,
    code: Code,
    name: String,
    vars: Vars,
    enabled: bool,
}
impl_command!(CreateChannelCommand, CommandType::CreateChannel);
into_event!(
    CreateChannelCommand,
    EventType::ChannelCreated,
    |cmd: &CreateChannelCommand| { cmd.channel_id }
);

#[derive(Debug)]
pub struct CreateChannelCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateChannelError {
    #[error("A channel with code {0} already exists")]
    CodeAlreadyExists(Code),
}

#[async_trait]
impl CommandHandler for CreateChannelCommandHandler {
    type Command = CreateChannelCommand;
    type Output = Channel;

    #[tracing::instrument(name = "create_channel_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        if exists_with_code(ctx.pool(), cmd.code()).await? {
            return Err(CommandBusError::ExpectedError(
                "Channel with code already exists",
            ));
        }

        ChannelBuilder::default()
            .id(cmd.channel_id)
            .code(cmd.code)
            .name(cmd.name)
            .vars(Json(cmd.vars))
            .enabled(true)
            .build()
            .tap_err(|e| {
                tracing::error!("Failed to build channel: {e}");
            })
            .map_err(anyhow::Error::from)?
            .save(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to save channel: {e}"))
            .map_err(CommandBusError::from)
    }
}

async fn exists_with_code<'tx>(poll: &PgPool, code: &Code) -> Result<bool, sqlx::Error> {
    Channel::exists(
        poll,
        ChannelsQueryBuilder::default()
            .code(Some(code.clone()))
            .build()
            .unwrap(),
    )
    .await
    .tap_err(|e| {
        tracing::error!(
            "Failed to checking if channel with code {} exists: {e}",
            code
        );
    })
}
