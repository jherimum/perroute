use crate::{
    api::{ConnectorPlugin, DispatcherPlugin},
    configuration::{
        Configuration, ConfigurationProperties, DefaultConfiguration, NilConfiguration,
    },
    types::{ConnectorPluginId, DispatchType, TemplateSupport},
};
use std::{collections::HashMap, marker::PhantomData, sync::Arc};

#[derive(Debug)]
pub struct LogConnectorPlugin {
    properties: Arc<dyn Configuration>,
    plugins: HashMap<DispatchType, Arc<dyn DispatcherPlugin>>,
}

impl Default for LogConnectorPlugin {
    fn default() -> Self {
        let mut plugins: HashMap<DispatchType, Arc<dyn DispatcherPlugin>> = HashMap::new();
        plugins.insert(
            DispatchType::Sms,
            Arc::new(LogDispatcherPlugin(
                DispatchType::Sms,
                ConfigurationProperties::default(),
            )),
        );
        plugins.insert(
            DispatchType::Push,
            Arc::new(LogDispatcherPlugin(
                DispatchType::Push,
                ConfigurationProperties::default(),
            )),
        );
        plugins.insert(
            DispatchType::Email,
            Arc::new(LogDispatcherPlugin(
                DispatchType::Email,
                ConfigurationProperties::default(),
            )),
        );

        Self {
            properties: Arc::new(DefaultConfiguration::new(
                vec![],
                PhantomData::<NilConfiguration>,
            )),
            plugins,
        }
    }
}

impl ConnectorPlugin for LogConnectorPlugin {
    fn id(&self) -> ConnectorPluginId {
        ConnectorPluginId::Log
    }

    fn configuration(&self) -> Arc<dyn Configuration> {
        self.properties.clone()
    }

    fn dispatchers(&self) -> &HashMap<DispatchType, Arc<dyn DispatcherPlugin>> {
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

    fn configuration(&self) -> Arc<dyn Configuration> {
        //&self.1
        todo!()
    }

    async fn dispatch(
        &self,
        req: &crate::api::DispatchRequest,
    ) -> Result<crate::api::DispatchResponse, crate::api::DispatchError> {
        todo!()
    }
}
