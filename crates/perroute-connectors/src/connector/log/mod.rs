use crate::{
    api::{
        BaseConnectorPlugin, BaseDispatcherPlugin, ConnectorPlugin, DispatchError, DispatchRequest,
        DispatchResponse,
    },
    configuration::DefaultConfiguration,
    types::{ConnectorPluginId, DispatchType, TemplateSupport},
};

pub fn log_connector_plugin() -> impl ConnectorPlugin {
    BaseConnectorPlugin::new(
        ConnectorPluginId::Log,
        Box::new(DefaultConfiguration::default()),
        vec![
            Box::new(BaseDispatcherPlugin::new(
                DispatchType::Email,
                TemplateSupport::None,
                Box::new(DefaultConfiguration::default()),
                |req| Box::pin(dispatch(req)),
            )),
            Box::new(BaseDispatcherPlugin::new(
                DispatchType::Sms,
                TemplateSupport::None,
                Box::new(DefaultConfiguration::default()),
                |req| Box::pin(dispatch(req)),
            )),
            Box::new(BaseDispatcherPlugin::new(
                DispatchType::Push,
                TemplateSupport::None,
                Box::new(DefaultConfiguration::default()),
                |req| Box::pin(dispatch(req)),
            )),
        ],
    )
}

pub async fn dispatch<'r>(_: &DispatchRequest<'r>) -> Result<DispatchResponse, DispatchError> {
    Ok(DispatchResponse::new(None, None))
}
