use crate::{
    api::{
        BaseConnectorPlugin, BaseDispatcherPlugin, ConnectorPlugin, DispatchError, DispatchRequest,
        DispatchResponse,
    },
    configuration::DefaultConfiguration,
    types::{ConnectorPluginId, DispatchType, TemplateSupport},
};
use std::sync::Arc;

pub fn log_connector_plugin() -> impl ConnectorPlugin {
    BaseConnectorPlugin::new(
        ConnectorPluginId::Log,
        Arc::new(DefaultConfiguration::default()),
        vec![
            Arc::new(BaseDispatcherPlugin::new(
                DispatchType::Email,
                TemplateSupport::None,
                Arc::new(DefaultConfiguration::default()),
                |req| Box::pin(dispatch(req)),
            )),
            Arc::new(BaseDispatcherPlugin::new(
                DispatchType::Sms,
                TemplateSupport::None,
                Arc::new(DefaultConfiguration::default()),
                |req| Box::pin(dispatch(req)),
            )),
            Arc::new(BaseDispatcherPlugin::new(
                DispatchType::Push,
                TemplateSupport::None,
                Arc::new(DefaultConfiguration::default()),
                |req| Box::pin(dispatch(req)),
            )),
        ],
    )
}

pub async fn dispatch<'r>(_: &DispatchRequest<'r>) -> Result<DispatchResponse, DispatchError> {
    Ok(DispatchResponse::new(None, None))
}
