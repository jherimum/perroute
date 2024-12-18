use super::DigesterResult;
use perroute_commons::types::template::Template;
use perroute_connectors::{DispatchRequest, ProviderPlugin, ProviderPluginRepository};
use perroute_storage::{
    models::{
        channel::Channel,
        dispatcher_log::{DispatcherError, DispatcherLog},
        message::Message,
        route::Route,
    },
    repository::Repository,
};
use std::sync::Arc;

pub(super) struct DispatchExecutor<R, PR> {
    pub repository: Arc<R>,
    pub plugin_repository: Arc<PR>,
}

impl<R: Repository, PR: ProviderPluginRepository> DispatchExecutor<R, PR> {
    async fn data(&self) -> DigesterResult<Vec<(Route, Channel, &dyn ProviderPlugin)>> {
        todo!()
    }

    pub async fn execute(
        &self,
        message: &Message,
        template: &Template,
    ) -> DigesterResult<Vec<DispatcherLog>> {
        let mut logs = vec![];
        let request = DispatchRequest::create(message.id(), message.recipient(), template)?;
        for (route, channel, plugin) in &self.data().await? {
            let cfg = channel.configuration().merge(route.configuration());
            let response = plugin.dispatch(&cfg, &request).await;
            match response {
                Ok(_) => {
                    logs.push(DispatcherLog::build_success(
                        message.id().clone(),
                        channel.provider_id().clone(),
                    ));
                    break;
                }
                Err(e) => {
                    logs.push(DispatcherLog::build_error(
                        message.id().clone(),
                        channel.provider_id().clone(),
                        DispatcherError {
                            code: "code".to_string(),
                            description: "des".to_string(),
                        },
                    ));
                }
            }
        }
        Ok(logs)
    }
}
