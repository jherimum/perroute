use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command,
};
use perroute_commons::{events::ChannelDeletedEvent, types::id::Id};
use perroute_storage::repository::{
    channels::{ChannelQuery, ChannelRepository},
    TransactedRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum DeleteChannelCommandError {}

impl_command!(DeleteChannelCommand, {
    channel_id: Id
});

pub struct DeleteChannelCommandHandler;

impl CommandHandler for DeleteChannelCommandHandler {
    type Command = DeleteChannelCommand;
    type Output = ();
    type ApplicationEvent = ChannelDeletedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        let result = ChannelRepository::delete(
            ctx.repository(),
            &ChannelQuery::ById(&cmd.inner().channel_id),
        )
        .await?;

        //Ok(())

        todo!()
    }
}
