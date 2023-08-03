use std::{collections::HashMap, sync::Arc};

use perroute_commons::types::dispatch_type::DispatcherType;

use crate::{
    plugin::{Configuration, DispatchError, DispatchRequest, DispatchResult, DispatcherPlugin},
    ConnectorPlugin,
};

#[derive(Debug)]
pub struct SmtpPlugin {
    id: &'static str,
    configuration: Configuration,
    plugins: HashMap<DispatcherType, Arc<dyn DispatcherPlugin>>,
}

impl Default for SmtpPlugin {
    fn default() -> Self {
        let mut plugins: HashMap<DispatcherType, Arc<dyn DispatcherPlugin>> = HashMap::new();
        plugins.insert(
            DispatcherType::Email,
            Arc::new(EmailDispatcherPlugin::default()),
        );
        Self {
            id: "smtp",
            configuration: Configuration::default(),
            plugins,
        }
    }
}

impl ConnectorPlugin for SmtpPlugin {
    fn id(&self) -> &str {
        &self.id
    }

    fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    fn dispatchers(&self) -> &HashMap<DispatcherType, Arc<dyn DispatcherPlugin>> {
        &self.plugins
    }
}

#[derive(Debug, Default)]
pub struct EmailDispatcherPlugin {
    configuration: Configuration,
}

impl DispatcherPlugin for EmailDispatcherPlugin {
    fn type_(&self) -> DispatcherType {
        DispatcherType::Email
    }

    fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    fn dispatch(&self, req: DispatchRequest) -> Result<DispatchResult, DispatchError> {
        todo!()
    }
}
