use std::sync::Arc;

use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use perroute_connectors::{
    api::{ConnectorPlugin, DispatcherPlugin},
    configuration::ConfigurationProperty,
    types::{ConnectorPluginId, DispatchType},
};
use serde::Serialize;

#[derive(Clone, Serialize, Debug, PartialEq, Eq)]
pub struct ConnectorPluginResource {
    id: ConnectorPluginId,
    configuration: Vec<ConfigurationProperty>,
    dispatcher_plugins: Vec<DispatcherPluginResource>,
}

#[derive(Clone, Serialize, Debug, PartialEq, Eq)]
pub struct DispatcherPluginResource {
    dispatch_type: DispatchType,
    configuration: Vec<ConfigurationProperty>,
}

impl From<Arc<dyn ConnectorPlugin>> for ConnectorPluginResource {
    fn from(value: Arc<dyn ConnectorPlugin>) -> Self {
        Self {
            id: value.id().clone(),
            configuration: value
                .configuration()
                .properties()
                .into_iter()
                .cloned()
                .collect(),
            dispatcher_plugins: Default::default(),
        }
    }
}

impl From<Arc<dyn DispatcherPlugin>> for DispatcherPluginResource {
    fn from(value: Arc<dyn DispatcherPlugin>) -> Self {
        Self {
            dispatch_type: value.dispatch_type(),
            configuration: value
                .configuration()
                .properties()
                .into_iter()
                .cloned()
                .collect(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<ConnectorPluginResource>> for Arc<dyn ConnectorPlugin> {
    fn build(&self, req: &HttpRequest) -> SingleResourceModel<ConnectorPluginResource> {
        SingleResourceModel {
            data: Some(ConnectorPluginResource::from(self.clone())),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Plugin(self.id()))
                .add(Linkrelation::Plugins, ResourceLink::Plugins)
                .as_url_map(req),
        }
    }
}

impl<'p> ResourceBuilder<CollectionResourceModel<ConnectorPluginResource>>
    for Vec<Arc<dyn ConnectorPlugin>>
{
    fn build(&self, req: &HttpRequest) -> CollectionResourceModel<ConnectorPluginResource> {
        CollectionResourceModel {
            data: self.into_iter().map(|c| c.build(req)).collect(),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Plugins)
                .as_url_map(req),
        }
    }
}
