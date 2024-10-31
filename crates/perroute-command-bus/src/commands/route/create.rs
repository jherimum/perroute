use bon::Builder;
use perroute_commons::types::{id::Id,  Configuration, Priority};
use perroute_storage::{models::route::Route, repository::TransactedRepository};
use crate::{bus::{Command, CommandBusContext, CommandHandler}, CommandBusResult};


#[derive(Debug, thiserror::Error)]
pub enum CreateRouteCommandError{
}

#[derive(Debug, Clone, Builder)]
pub struct CreateRouteCommand {
     channel_id: Id,
     message_type_id: Id,
     configuration: Configuration,
     priority: Priority,
     enabled: bool,
    
}

impl Command for CreateRouteCommand {
}


pub struct CreateRouteCommandHandler;

impl CommandHandler for CreateRouteCommandHandler {
    type Command = CreateRouteCommand;
    type Output = Route;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        todo!()
    }

}



