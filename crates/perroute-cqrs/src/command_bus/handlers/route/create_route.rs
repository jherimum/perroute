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
use perroute_connectors::Plugins;
use perroute_storage::{
    models::{
        channel::{Channel, ChannelQuery},
        route::{Route, RouteBuilder},
        schema::{Schema, SchemasQuery},
    },
    query::FetchableModel,
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateRouteCommand {
    id: Id,
    channel_id: Id,
    schema_id: Id,
    properties: Properties,
}

impl_command!(CreateRouteCommand, CommandType::CreateRoute);
into_event!(CreateRouteCommand);

#[derive(Debug)]
pub struct CreateRouteCommandHandler {
    plugins: Plugins,
}

#[async_trait::async_trait]
impl CommandHandler for CreateRouteCommandHandler {
    type Command = CreateRouteCommand;
    type Output = Route;

    #[tracing::instrument(name = "create_route_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let channel = Channel::find(ctx.pool(), ChannelQuery::with_id(cmd.channel_id))
            .await
            .unwrap()
            .unwrap();

        let conn = channel.connection(ctx.pool()).await.unwrap();
        let plugin = conn.plugin(&self.plugins).unwrap();
        let disp = plugin.dispatcher(channel.dispatch_type()).unwrap();

        let props = channel.properties().merge(&cmd.properties);
        disp.configuration().validate(&props).unwrap();

        let schema = Schema::find(ctx.pool(), SchemasQuery::with_id(cmd.schema_id))
            .await
            .unwrap()
            .unwrap();

        Ok(RouteBuilder::default()
            .id(cmd.id)
            .channel_id(*channel.id())
            .message_type_id(*schema.message_type_id())
            .business_unit_id(*channel.business_unit_id())
            .schema_id(*schema.id())
            .properties(props)
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .unwrap())
    }
}
