use perroute_commons::{
    events::{ApplicationEventData, MessageCreatedEvent},
    template::{context::TemplateRenderContext, TemplateError, TemplateRender},
    types::{id::Id, template::Template, MessageStatus, ProviderId},
};
use perroute_connectors::{PluginDispatchError, ProviderPluginError, ProviderPluginRepository};
use perroute_storage::{
    models::{
        dispatcher_log::DispatcherLog, message::Message, template_assignment::TemplateAssignment,
    },
    repository::{
        message::{MessageQuery, MessageRepository},
        template_assignment::{QueryForDispatch, TemplateAssignmentRepository},
        Repository,
    },
};
use stack::DispatchStack;
use std::sync::Arc;

mod stack;

type DigesterResult<T> = Result<T, DigesterError>;

#[derive(Debug, thiserror::Error)]
pub enum DigesterError {
    #[error("{0}")]
    RepositoryError(#[from] perroute_storage::repository::Error),

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
}

pub struct Dispatcher<R, TR, PR> {
    pub template_render: Arc<TR>,
    pub repository: Arc<R>,
    pub plugin_repository: Arc<PR>,
    pub event: ApplicationEventData<MessageCreatedEvent>,
}

impl<R: Repository, TR: TemplateRender, PR: ProviderPluginRepository> Dispatcher<R, TR, PR> {
    async fn retrieve_message(&self) -> DigesterResult<Message> {
        match MessageRepository::find(
            self.repository.as_ref(),
            &MessageQuery::ById(self.event.entity_id()),
        )
        .await?
        {
            Some(message) if *message.status() != MessageStatus::Pending => Err(
                DigesterError::InvalidMessageStatus(message.id().clone(), message.status().clone()),
            ),
            Some(message) => Ok(message),
            None => Err(DigesterError::MessageNotFound(
                self.event.entity_id().clone(),
            )),
        }
    }

    async fn render_template(&self, message: &Message) -> DigesterResult<Template> {
        let template_assignment = self
            .template_assignment(&message)
            .await?
            .ok_or(DigesterError::NoTemplateAssignmentElegible)?;
        let template = self.template(&message, &template_assignment).await?;
        let ctx = self.context(&message, &template_assignment).await?;
        template
            .render(self.template_render.as_ref(), &ctx)
            .map_err(DigesterError::from)
    }

    async fn template_assignment(
        &self,
        message: &Message,
    ) -> DigesterResult<Option<TemplateAssignment>> {
        let query = QueryForDispatch::builder()
            .message_type_id(message.message_type_id())
            .business_unit_id(message.business_unit_id())
            .dispatch_type(message.dispatch_type())
            .reference_date(
                message
                    .scheduled_at()
                    .as_ref()
                    .unwrap_or(message.created_at()),
            )
            .build();

        Ok(TemplateAssignmentRepository::find(self.repository.as_ref(), query.into()).await?)
    }

    async fn template(
        &self,
        message: &Message,
        template_assignment: &TemplateAssignment,
    ) -> DigesterResult<Template> {
        todo!()
    }

    async fn context(
        &self,
        message: &Message,
        template_assignment: &TemplateAssignment,
    ) -> DigesterResult<TemplateRenderContext> {
        todo!()
    }

    async fn stack<'s>(
        &self,
        message: &'s Message,
        template: &'s Template,
    ) -> DigesterResult<DispatchStack<'s>> {
        let dispatchers = vec![];
        Ok(DispatchStack {
            message,
            template,
            data: dispatchers,
        })
    }

    async fn process(&self, message: &Message) -> DigesterResult<Vec<DispatcherLog>> {
        let rendered_template = self.render_template(&message).await?;
        let stack = self.stack(message, &rendered_template).await?;
        Ok(stack.execute().await?)
    }

    pub async fn execute(self) -> DigesterResult<()> {
        let mut message = match self.retrieve_message().await {
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

        match self.process(&message).await {
            Ok(logs) if logs.iter().any(|l| *l.success()) => {
                message = message.set_status(MessageStatus::Dispatched);
                Ok(())
            }

            Ok(logs) => {
                message = message.set_status(MessageStatus::Failed);
                Ok(())
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
