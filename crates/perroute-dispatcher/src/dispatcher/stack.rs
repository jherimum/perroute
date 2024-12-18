use perroute_commons::types::template::Template;
use perroute_connectors::{DispatchRequest, ProviderPlugin};
use perroute_storage::models::{
    channel::Channel,
    dispatcher_log::{DispatcherError, DispatcherLog},
    message::Message,
    route::Route,
};

use super::DigesterResult;

pub(super) struct DispatchStack<'s> {
    pub message: &'s Message,
    pub template: &'s Template,
    pub data: Vec<(Route, Channel, &'s dyn ProviderPlugin)>,
}

impl DispatchStack<'_> {
    pub async fn execute(&self) -> DigesterResult<Vec<DispatcherLog>> {
        let mut logs = vec![];
        let request =
            DispatchRequest::create(self.message.id(), self.message.recipient(), self.template)?;
        for (route, channel, plugin) in &self.data {
            let cfg = channel.configuration().merge(route.configuration());
            let response = plugin.dispatch(&cfg, &request).await;
            match response {
                Ok(_) => {
                    logs.push(DispatcherLog::build_success(
                        self.message.id().clone(),
                        channel.provider_id().clone(),
                    ));
                    break;
                }
                Err(e) => {
                    logs.push(DispatcherLog::build_error(
                        self.message.id().clone(),
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
