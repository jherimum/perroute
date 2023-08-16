use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, id::Id, properties::Properties};
use perroute_storage::{
    models::route::{Route, RouteQueryBuilder},
    query::FetchableModel,
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct UpdateRouteCommand {
    id: Id,
    properties: Properties,
}

impl_command!(UpdateRouteCommand, CommandType::UpdateRoute);
into_event!(UpdateRouteCommand);

#[derive(Debug)]
pub struct UpdateRouteCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for UpdateRouteCommandHandler {
    type Command = UpdateRouteCommand;
    type Output = Route;

    #[tracing::instrument(name = "update_route_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let route = Route::find_one(ctx.pool(), RouteQueryBuilder::default().build().unwrap())
            .await
            .unwrap();

        Ok(route)
    }
}
