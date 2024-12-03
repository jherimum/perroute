use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
};
use bon::Builder;
use perroute_commons::{events::RouteDeletedEvent, types::id::Id};
use perroute_storage::repository::{
    routes::{RouteQuery, RouteRepository},
    TransactedRepository,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum DeleteRouteCommandError {}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct DeleteRouteCommand {
    id: Id,
}

impl Command for DeleteRouteCommand {
    fn event_type(&self) -> perroute_commons::events::EventType {
        perroute_commons::events::EventType::RouteDeleted
    }

    fn entity_id(&self) -> &Id {
        &self.id
    }
}

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
        let deleted = RouteRepository::delete(ctx.repository(), &RouteQuery::ById(&cmd.inner().id))
            .await?
            > 0;

        //Ok(())
        todo!()
    }
}
