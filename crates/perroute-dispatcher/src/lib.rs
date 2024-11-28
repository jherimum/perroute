pub mod dispatcher;
pub mod template;

use dispatcher::{MessageDispatcherError, MessageDispatchers};
use perroute_commons::{
    template::TemplateRender,
    types::{id::Id, MessageStatus},
};
use perroute_connectors::{DispatchRequest, ProviderPluginError, ProviderPluginRepository};
use perroute_storage::{
    models::{
        business_unit::BusinessUnit, dispatcher_log::DispatcherLog, message::Message,
        message_type::MessageType,
    },
    repository::{
        message::{MessageQuery, MessageRepository},
        Repository,
    },
};
use tap::TapOptional;
use template::{TemplateError, Templates};

#[derive(Debug, thiserror::Error)]
pub enum DispatcherError {
    #[error("Message with id {0} not found")]
    MessageNotFound(Id),

    #[error("Invalid message status")]
    InvalidMessageStatus,

    #[error("Repository error: {0}")]
    RepositoryError(#[from] perroute_storage::repository::Error),

    #[error("Dispatcher error: {0}")]
    MessageDispatcherError(#[from] MessageDispatcherError),

    #[error("Template error: {0}")]
    TemplateError(#[from] TemplateError),

    #[error("Plugin error: {0}")]
    ProviderPluginError(#[from] ProviderPluginError),
}

pub type DispatchData = (BusinessUnit, MessageType);

pub struct Dispatcher<R, PR, TR> {
    repository: R,
    templates: Templates<R, TR>,
    message_dispatchers: MessageDispatchers<PR>,
}

impl<R: Repository + Clone, PR: ProviderPluginRepository, TR: TemplateRender>
    Dispatcher<R, PR, TR>
{
    pub fn new(repository: R, plugin_repository: PR, template_render: TR) -> Self {
        Dispatcher {
            repository: repository.clone(),
            templates: Templates::new(repository, template_render),
            message_dispatchers: MessageDispatchers::new(plugin_repository),
        }
    }

    pub async fn dispatch(&self, message_id: &Id) -> Result<Message, DispatcherError> {
        log::info!("Dispatching message with id {}", message_id);

        let mut message =
            MessageRepository::find(&self.repository, &MessageQuery::ById(message_id))
                .await?
                .tap_none(|| log::error!("Message with id {} not found", message_id))
                .ok_or(DispatcherError::MessageNotFound(message_id.into()))?;

        if *message.status() != MessageStatus::Pending {
            log::error!("Message with id {} is not pending", message.id());
            return Err(DispatcherError::InvalidMessageStatus);
        }

        let result: Result<Vec<DispatcherLog>, DispatcherError> = {
            let template = self.templates.find_and_render(&message).await?;
            let request = DispatchRequest::create(message.id(), message.recipient(), &template)?;
            Ok(self.message_dispatchers.dispatch(request).await?)
        };

        let tx = self.repository.begin().await?;
        message = match result {
            Ok(logs) => message,
            Err(err) => message,
        };

        //let message = MessageRepository::update(&tx, message).await?;
        //DispatcherLogRepository::save_all(&tx, logs);

        Ok(MessageRepository::update(&self.repository, message).await?)
    }
}
