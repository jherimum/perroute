use crate::api::{
    ConfigurationProperties, ConnectorPlugin, ConnectorPluginId, DispatchType, DispatcherPlugin,
    TemplateSupport,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct LogConnectorPlugin {
    properties: ConfigurationProperties,
    plugins: HashMap<DispatchType, Box<dyn DispatcherPlugin>>,
}

impl Default for LogConnectorPlugin {
    fn default() -> Self {
        let mut plugins: HashMap<DispatchType, Box<dyn DispatcherPlugin>> = HashMap::new();
        plugins.insert(
            DispatchType::Sms,
            Box::new(LogDispatcherPlugin(
                DispatchType::Sms,
                ConfigurationProperties::default(),
            )),
        );
        plugins.insert(
            DispatchType::Push,
            Box::new(LogDispatcherPlugin(
                DispatchType::Push,
                ConfigurationProperties::default(),
            )),
        );
        plugins.insert(
            DispatchType::Email,
            Box::new(LogDispatcherPlugin(
                DispatchType::Email,
                ConfigurationProperties::default(),
            )),
        );

        Self {
            properties: Default::default(),
            plugins,
        }
    }
}

impl ConnectorPlugin for LogConnectorPlugin {
    fn id(&self) -> ConnectorPluginId {
        ConnectorPluginId::Log
    }

    fn configuration(&self) -> &ConfigurationProperties {
        &self.properties
    }

    fn dispatchers(&self) -> &HashMap<DispatchType, Box<dyn DispatcherPlugin>> {
        &self.plugins
    }
}

#[derive(Debug)]
pub struct LogDispatcherPlugin(DispatchType, ConfigurationProperties);

#[async_trait::async_trait]
impl DispatcherPlugin for LogDispatcherPlugin {
    fn template_support(&self) -> TemplateSupport {
        TemplateSupport::Mandatory
    }

    fn dispatch_type(&self) -> DispatchType {
        self.0
    }

    fn configuration(&self) -> &ConfigurationProperties {
        &self.1
    }

    async fn dispatch(
        &self,
        req: &crate::api::DispatchRequest,
    ) -> Result<crate::api::DispatchResponse, crate::api::DispatchError> {
        todo!()
    }
}
