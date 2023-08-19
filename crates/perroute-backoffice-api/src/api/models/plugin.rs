use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use perroute_connectors::{
    api::{ConnectorPlugin, DispatcherPlugin},
    configuration::ConfigurationProperty,
};
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Serialize, Debug, PartialEq, Eq)]
pub struct ConnectorPluginResource {
    id: String,
    configuration: Vec<ConfigurationProperty>,
    dispatcher_plugins: Vec<DispatcherPluginResource>,
}

#[derive(Clone, Serialize, Debug, PartialEq, Eq)]
pub struct DispatcherPluginResource {
    dispatch_type: String,
    configuration: Vec<ConfigurationProperty>,
}

impl From<Arc<dyn ConnectorPlugin>> for ConnectorPluginResource {
    fn from(value: Arc<dyn ConnectorPlugin>) -> Self {
        Self {
            id: value.id().into(),
            configuration: value
                .configuration()
                .properties()
                .into_iter()
                .cloned()
                .collect(),
            dispatcher_plugins: value.dispatchers().into_iter().map(Into::into).collect(),
        }
    }
}

impl From<&dyn DispatcherPlugin> for DispatcherPluginResource {
    fn from(value: &dyn DispatcherPlugin) -> Self {
        Self {
            dispatch_type: value.dispatch_type().to_string(),
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

impl ResourceBuilder<CollectionResourceModel<ConnectorPluginResource>>
    for Vec<Arc<dyn ConnectorPlugin>>
{
    fn build(&self, req: &HttpRequest) -> CollectionResourceModel<ConnectorPluginResource> {
        CollectionResourceModel {
            data: self.iter().map(|c| c.build(req)).collect(),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Plugins)
                .as_url_map(req),
        }
    }
}
