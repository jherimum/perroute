use perroute_connectors::{DispatchRequest, ProviderPlugin, ProviderPluginRepository};
use perroute_storage::{
    models::{channel::Channel, dispatcher_log::DispatcherLog, message::Message, route::Route},
    repository::Repository,
};

pub struct MessageDispatchers<R, PR> {
    repository: R,
    plugin_repository: PR,
}

impl<R: Repository, PR: ProviderPluginRepository> MessageDispatchers<R, PR> {
    pub fn new(repository: R, plugin_repository: PR) -> Self {
        Self {
            repository,
            plugin_repository,
        }
    }

    pub async fn stack(
        &self,
        message: &Message,
    ) -> Result<Vec<MessageDispatcher<'_>>, perroute_storage::repository::Error> {
        todo!()
    }
}

pub struct MessageDispatcher<'d> {
    route: &'d Route,
    channel: &'d Channel,
    plugin: &'d dyn ProviderPlugin,
}

impl<'d> MessageDispatcher<'d> {
    pub fn new(route: &'d Route, channel: &'d Channel, plugin: &'d dyn ProviderPlugin) -> Self {
        MessageDispatcher {
            route,
            channel,
            plugin,
        }
    }

    pub async fn dispatch(
        &self,
        req: &DispatchRequest,
    ) -> Result<DispatcherLog, perroute_storage::repository::Error> {
        let configuration = self
            .channel
            .configuration()
            .merge(self.route.configuration());

        let plugin = self.plugin.dispatch(&configuration, req).await;

        todo!("build message dispatch")
    }
}

/*

async fn retrieve_routes(
        &self,
        message: &Message,
    ) -> Result<Vec<(Route, Channel)>, DispatcherError> {
        let mut routes = RouteRepository::query(
            &self.repository,
            &RouteQuery::ActiveByBusinessUnitAndDispatchType(
                &ActiveByBusinessUnitAndDispatchTypeQuery {
                    business_unit_id: message.business_unit_id(),
                    message_type_id: message.message_type_id(),
                    dispatch_type: message.dispatch_type(),
                },
            ),
        )
        .await?;

        let channel_ids = routes
            .iter()
            .map(|r| r.channel_id())
            .collect::<Vec<_>>()
            .as_slice();

        let channels =
            ChannelRepository::query(&self.repository, &ChannelQuery::ActiveByIds(&channel_ids));

        if routes.is_empty() {
            log::warn!(
                "No routes found for message with id {} and dispatch type {}",
                message.id(),
                message.dispatch_type()
            );
            return Err(DispatcherError::NoRoutesConfigured);
        }
        routes.sort_by(|r1, r2| r1.priority().cmp(r2.priority()));

        Ok(routes)
    }

*/
