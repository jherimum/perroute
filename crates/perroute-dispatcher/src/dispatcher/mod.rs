use perroute_commons::{
    events::{ApplicationEventData, MessageCreatedEvent},
    template::{TemplateError, TemplateRender},
    types::{id::Id, MessageStatus, ProviderId},
};
use perroute_connectors::{
    PluginDispatchError, ProviderPluginError, ProviderPluginRepository,
};
use perroute_storage::{
    models::{dispatcher_log::DispatcherLog, message::Message},
    repository::{
        dispatcher_log::DispatcherLogRepository,
        message::{MessageQuery, MessageRepository},
        Repository, TransactedRepository,
    },
};
use stack::DispatchExecutor;
use std::sync::Arc;
use template::TemplateResolver;

mod stack;
mod template;

type DigesterResult<T> = Result<T, DigesterError>;

#[derive(Debug, thiserror::Error)]
pub enum DigesterError {
    //#[error("{0}")]
    //RepositoryError(#[from] perroute_storage::repository::Error),
    #[error("Message not found: {0}")]
    MessageNotFound(Id),

    #[error("Invalid message status: {0:?}")]
    InvalidMessageStatus(Id, MessageStatus),

    #[error("{0}")]
    TemplaterError(#[from] TemplateError),

    #[error("{0}")]
    PluginDispatchError(#[from] PluginDispatchError),

    #[error("No template assignment elegible")]
    NoTemplateAssignmentElegible,

    #[error("No route elegible")]
    NoRouteElegible,

    #[error("Provider plugin not found: {0}")]
    ProviderPluginNotFound(ProviderId),

    #[error("{0}")]
    ProviderPluginError(#[from] ProviderPluginError),

    #[error("{0}")]
    TemplateError(#[from] crate::dispatcher::template::Error),
}

pub struct Dispatcher<R, TR, PR> {
    repository: Arc<R>,
    template_resolver: TemplateResolver<R, TR>,
    dispatch_executor: DispatchExecutor<R, PR>,
    event: ApplicationEventData<MessageCreatedEvent>,
}

impl<R: Repository, TR: TemplateRender, PR: ProviderPluginRepository>
    Dispatcher<R, TR, PR>
{
    pub fn new(
        repository: Arc<R>,
        plugin_repository: Arc<PR>,
        template_render: Arc<TR>,
        event: ApplicationEventData<MessageCreatedEvent>,
    ) -> Self {
        Dispatcher {
            repository: repository.clone(),
            event,
            template_resolver: TemplateResolver {
                repository: repository.clone(),
                template_render: template_render.clone(),
            },
            dispatch_executor: DispatchExecutor {
                repository,
                plugin_repository,
            },
        }
    }

    async fn retrieve_message(&self) -> DigesterResult<Message> {
        match MessageRepository::find(
            self.repository.as_ref(),
            &MessageQuery::ById(self.event.entity_id()),
        )
        .await
        .unwrap()
        {
            Some(message) if *message.status() != MessageStatus::Pending => {
                Err(DigesterError::InvalidMessageStatus(
                    message.id().clone(),
                    message.status().clone(),
                ))
            }
            Some(message) => Ok(message),
            None => Err(DigesterError::MessageNotFound(
                self.event.entity_id().clone(),
            )),
        }
    }

    async fn process(
        &self,
        message: &Message,
    ) -> DigesterResult<Vec<DispatcherLog>> {
        let rendered_template = self.template_resolver.resolve(message).await?;
        self.dispatch_executor
            .execute(message, &rendered_template.unwrap())
            .await
    }

    pub async fn execute(self) -> DigesterResult<()> {
        let message = match self.retrieve_message().await {
            Ok(message) => message,
            Err(DigesterError::MessageNotFound(id)) => {
                log::warn!("Message not found: {:?}", id);
                return Ok(());
            }
            Err(DigesterError::InvalidMessageStatus(_, _)) => {
                log::warn!("Message already dispatched");
                return Ok(());
            }
            Err(e) => return Err(e),
        };

        let (message, logs) = match self.process(&message).await {
            Ok(logs) if logs.iter().any(|l| *l.success()) => {
                (message.set_status(MessageStatus::Dispatched), logs)
            }

            Ok(logs) => (message.set_status(MessageStatus::Failed), logs),
            Err(e) => {
                return Err(e);
            }
        };

        let tx = self.repository.begin().await.unwrap();

        match MessageRepository::update(&tx, message).await {
            Ok(_) => {
                match DispatcherLogRepository::save_all(
                    self.repository.as_ref(),
                    logs,
                )
                .await
                {
                    Ok(_) => {
                        tx.commit().await.unwrap();
                    }
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        todo!()
                        //return Err(e.into());
                    }
                }
            }
            Err(_) => {
                tx.rollback().await.unwrap();
            }
        }

        Ok(())
    }
}
