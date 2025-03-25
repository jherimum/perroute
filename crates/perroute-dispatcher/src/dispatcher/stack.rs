use std::sync::Arc;
use perroute_connectors::{
    DispatchRequest, ProviderPlugin, ProviderPluginRepository,
};
use perroute_storage::{
    models::{
        channel::Channel, dispatcher_log::DispatcherLog, message::Message,
        route::Route,
    },
    repository::route::RouteRepository,
};
use super::DispatchError;

#[derive(Debug, Clone)]
pub struct Stacks<R> {
    repository: R,
    plugin_repository: ProviderPluginRepository,
}

impl<R: RouteRepository> Stacks<R> {
    pub fn new(
        repository: R,
        plugin_repository: ProviderPluginRepository,
    ) -> Self {
        Self {
            repository,
            plugin_repository,
        }
    }

    pub async fn create(
        &self,
        message: &Message,
    ) -> Result<DispatchStack, DispatchError> {
        match RouteRepository::routes_to_dispatch(
            &self.repository,
            message.business_unit_id(),
            message.message_type_id(),
            message.dispatch_type(),
        )
        .await
        {
            Ok(routes) if routes.is_empty() => {
                Err(DispatchError::NoRouteEligible)
            }
            Ok(routes) => Ok(DispatchStack::default()),

            Err(e) => Err(DispatchError::from(e)),
        }
    }
}

#[derive(Default)]
pub struct DispatchStack {
    stack: Vec<(Route, Channel, Arc<dyn ProviderPlugin>)>,
}

impl DispatchStack {
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub async fn dispatch(
        self,
        request: DispatchRequest<'_>,
    ) -> Vec<DispatcherLog> {
        // let mut logs = vec![];

        // for (route, channel, plugin) in &self.stack {
        //     let cfg = channel.configuration().merge(route);
        //     logs.push(plugin.dispatch(&cfg, &request).await);

        //     if logs.last().is_some_and(|log| log.is_ok()) {
        //         break;
        //     }
        // }

        // logs

        todo!()
    }
}
