use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command,
};
use perroute_commons::{events::RouteDeletedEvent, types::id::Id};
use perroute_storage::repository::{
    routes::{RouteQuery, RouteRepository},
    TransactedRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum DeleteRouteCommandError {}

impl_command!(DeleteRouteCommand, {
    route_id: Id,
});

pub struct DeleteRouteCommandHandler;

impl CommandHandler for DeleteRouteCommandHandler {
    type Command = DeleteRouteCommand;
    type Output = ();
    type ApplicationEvent = RouteDeletedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        let deleted =
            RouteRepository::delete(ctx.repository(), &RouteQuery::ById(&cmd.inner().route_id))
                .await?
                > 0;

        //Ok(())
        todo!()
    }
}
