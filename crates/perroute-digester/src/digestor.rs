use perroute_commons::{
    events::{ApplicationEventData, MessageCreatedEvent},
    template::{context::TemplateRenderContext, TemplateError, TemplateRender},
    types::{id::Id, template::Template, MessageStatus},
};
use perroute_connectors::{DispatchRequest, PluginDispatchError, ProviderPluginRepository};
use perroute_storage::{
    models::{
        channel::Channel, dispatcher_log::DispatcherLog, message::Message, route::Route,
        template_assignment::TemplateAssignment,
    },
    repository::{
        message::{MessageQuery, MessageRepository},
        Repository,
    },
};
use std::sync::Arc;

pub struct Digesters<R, TR, PR> {
    repository: Arc<R>,
    template_render: Arc<TR>,
    plugin_repository: Arc<PR>,
}

impl<
        R: Repository + Sync + Send,
        TR: TemplateRender + Send + Sync,
        PR: ProviderPluginRepository + Send + Sync,
    > Digesters<R, TR, PR>
{
    pub fn new(repository: R, template_render: TR, plugin_repository: PR) -> Self {
        Digesters {
            repository: Arc::new(repository),
            template_render: Arc::new(template_render),
            plugin_repository: Arc::new(plugin_repository),
        }
    }

    pub fn create(&self, event: ApplicationEventData<MessageCreatedEvent>) -> Digester<R, TR, PR> {
        todo!()
    }
}

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
}

pub struct Digester<R, TR, PR> {
    pub template_render: Arc<TR>,
    pub repository: Arc<R>,
    pub event: ApplicationEventData<MessageCreatedEvent>,
    pub plugin_repository: Arc<PR>,
}

impl<
        R: Repository + Send + Sync,
        TR: TemplateRender,
        PR: ProviderPluginRepository + Send + Sync,
    > Digester<R, TR, PR>
{
    async fn message(&self) -> Result<Message, DigesterError> {
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
            None => {
                return Err(DigesterError::MessageNotFound(
                    self.event.entity_id().clone(),
                ))
            }
        }
    }

    async fn render_template(&self, message: &Message) -> Result<Template, DigesterError> {
        let template_assignment = self.template_assignment(&message).await?;
        let template = self.template(&message, &template_assignment).await?;
        let ctx = self.context(&message, &template_assignment).await?;
        let rendered_template = template
            .render(self.template_render.as_ref(), &ctx)
            .unwrap();

        todo!()
    }

    async fn template_assignment(
        &self,
        message: &Message,
    ) -> Result<TemplateAssignment, DigesterError> {
        todo!()
    }

    async fn template(
        &self,
        message: &Message,
        template_assignment: &TemplateAssignment,
    ) -> Result<Template, DigesterError> {
        todo!()
    }

    async fn context(
        &self,
        message: &Message,
        template_assignment: &TemplateAssignment,
    ) -> Result<TemplateRenderContext, DigesterError> {
        todo!()
    }

    async fn routes(&self, message: &Message) -> Result<Vec<(Route, Channel)>, DigesterError> {
        todo!()
    }

    fn plugin(&self, channel: &Channel) -> &dyn perroute_connectors::ProviderPlugin {
        self.plugin_repository.get(channel.provider_id()).unwrap()
    }

    async fn dispatch(
        &self,
        message: &Message,
        template: &Template,
        route: &Route,
        channel: &Channel,
    ) -> Result<DispatcherLog, DigesterError> {
        let plugin = self.plugin(channel);
        let cfg = channel.configuration().merge(route.configuration());
        plugin
            .dispatch(
                cfg,
                DispatchRequest::create(message.id(), message.recipient(), template).unwrap(),
            )
            .await
            .unwrap();

        todo!()
    }

    pub async fn execute(self) -> Result<(), DigesterError> {
        let message = self.message().await?;

        let tx = self.repository.as_ref().begin().await;

        let rendered_template = self.render_template(&message).await?;

        for (route, channel) in self.routes(&message).await? {
            let result = self
                .dispatch(&message, &rendered_template, &route, &channel)
                .await
                .unwrap();
        }
        todo!()
    }
}
