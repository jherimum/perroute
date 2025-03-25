use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::{Command},
    impl_command,
};
use perroute_commons::{events::RouteDeletedEvent, types::id::Id};

#[derive(Debug, thiserror::Error)]
pub enum DeleteRouteCommandError {}

impl_command!(DeleteRouteCommand, {
    route_id: Id,
});

pub struct DeleteRouteCommandHandler;

impl CommandHandler for DeleteRouteCommandHandler {
    type Command = DeleteRouteCommand;
    type Output = ();
    type E = RouteDeletedEvent;

    async fn handle<
        C: AsRef<perroute_storage::active_record::datasource::Connection>,
    >(
        &self,

        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> CommandHandlerResult<Self::Output> {
        // let deleted = RouteRepository::delete(
        //     ctx.repository(),
        //     &RouteQuery::ById(&cmd.inner().route_id),
        // )
        // .await?
        //     > 0;

        // //Ok(())
        todo!()
    }
}
