mod dispatcher;
mod template;

use dispatcher::MessageDispatchers;
use perroute_commons::{
    template::{TemplateError, TemplateRender},
    types::{id::Id, template::Template, MessageStatus, ProviderId},
};
use perroute_connectors::{DispatchRequest, ProviderPluginRepository};
use perroute_storage::{
    models::{business_unit::BusinessUnit, message::Message, message_type::MessageType},
    repository::{
        business_units::BusinessUnitRepository,
        message::{MessageQuery, MessageRepository},
        message_types::MessageTypeRepository,
        Repository,
    },
};
use tap::TapOptional;
use template::Templates;

#[derive(Debug, thiserror::Error)]
pub enum DispatcherError {
    #[error("Repository error: {0}")]
    RepositoryError(#[from] perroute_storage::repository::Error),

    #[error("Message {0} not found")]
    MessageNotFound(Id),

    #[error("Message {0} not elegible to be dispatched. Actual status is {1}")]
    MessageNotElegible(Id, MessageStatus),

    #[error("There is no active routes configured for the message")]
    NoRoutesConfigured,

    #[error("No template found")]
    NoTemplateFound,

    #[error("Channel not found")]
    ChannelNotFound,

    #[error("Template error: {0}")]
    TemplateError(#[from] TemplateError),

    #[error("Plugin with id {0} not found")]
    PluginNotFound(ProviderId),

    #[error("Invalid request")]
    InvalidRequest(String),
}

pub type DispatchData = (Message, BusinessUnit, MessageType);

pub struct Dispatcher<R, PR, TR> {
    repository: R,
    templates: Templates<R, TR>,
    message_dispatchers: MessageDispatchers<R, PR>,
}

impl<R: Repository + Clone, PR: ProviderPluginRepository, TR: TemplateRender>
    Dispatcher<R, PR, TR>
{
    pub fn new(repository: R, plugin_repository: PR, template_render: TR) -> Self {
        Dispatcher {
            repository: repository.clone(),
            templates: Templates::new(repository.clone(), template_render),
            message_dispatchers: MessageDispatchers::new(repository, plugin_repository),
        }
    }

    async fn fetch_data(&self, message_id: &Id) -> Result<DispatchData, DispatcherError> {
        let message = MessageRepository::query(&self.repository, &MessageQuery::ById(message_id))
            .await?
            .tap_none(|| log::warn!("Message with id {} not found", message_id))
            .ok_or(DispatcherError::MessageNotFound(message_id.clone()))?;

        let bu = BusinessUnitRepository::get(&self.repository, message.business_unit_id()).await?;

        let message_type =
            MessageTypeRepository::find_message_type(&self.repository, message.message_type_id())
                .await?;

        todo!()
    }

    pub async fn dispatch(&self, message_id: &Id) -> Result<(), DispatcherError> {
        log::info!("Dispatching message with id {}", message_id);

        let (message, business_unit, message_type) = self.fetch_data(message_id).await?;
        let routes = self.message_dispatchers.stack(&message).await?;
        let template = self
            .templates
            .find_and_render(&message, &business_unit, message_type)
            .await?;

        let request = Self::build_request(&message, &template).await?;

        for route in routes {
            let log = route.dispatch(&request).await?;
        }

        Ok(())
    }

    async fn build_request(
        message: &Message,
        rendered_template: &Template,
    ) -> Result<DispatchRequest, DispatcherError> {
        DispatchRequest::try_from((
            message.id(),
            message.recipient().clone(),
            rendered_template.clone(),
        ))
        .map_err(|error| DispatcherError::InvalidRequest(error.to_string()))
    }
}
