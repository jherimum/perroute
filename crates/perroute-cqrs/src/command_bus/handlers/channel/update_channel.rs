use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::{actor::Actor, id::Id, vars::Vars};
use perroute_storage::{
    models::channel::{Channel, ChannelsQueryBuilder},
    query::FetchableModel,
};
use sqlx::types::Json;
use tap::TapFallible;

command!(
    UpdateChannelCommand,
    CommandType::UpdateChannel,
    channel_id: Id,
    name: String,
    vars: Vars,
    enabled: bool
);
into_event!(UpdateChannelCommand);

#[derive(Debug, new)]
pub struct UpdateChannelCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum UpdateChannelError {
    #[error("Channel with id {0} nor found")]
    ChannelNotFound(Id),
}

#[async_trait]
impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;
    type Output = Channel;

    #[tracing::instrument(name = "update_channel_handler", skip(self, ctx))]
    async fn handle<'ctx>(
        &self,
        ctx: &mut CommandBusContext<'ctx>,
        actor: &Actor,
        command: Self::Command,
    ) -> Result<Channel, CommandBusError> {
        let channel = Channel::find(
            ctx.tx(),
            ChannelsQueryBuilder::default()
                .id(Some(*command.channel_id()))
                .build()
                .unwrap(),
        )
        .await?
        .unwrap();

        channel
            .set_name(command.name().clone())
            .set_vars(Json(command.vars().clone()))
            .set_enabled(*command.enabled())
            .update(ctx.tx())
            .await
            .tap_err(|e| {
                tracing::error!("Error while updating channel {}: {e}", command.channel_id());
            })
            .map_err(CommandBusError::from)
    }
}
