use error::DispatchError;
use perroute_commons::{
    events::{ApplicationEventData, MessageCreatedEvent},
    types::{MessageStatus, Timestamp},
};
use perroute_connectors::{DispatchRequest, ProviderPluginRepository};
use perroute_storage::{
    active_record::message::MessageQuery,
    models::{dispatcher_log::DispatcherLog, message::Message},
    repository::{
        business_unit::BusinessUnitRepository,
        dispatcher_log::DispatcherLogRepository, message::MessageRepository,
        message_type::MessageTypeRepository, route::RouteRepository,
        template_assignment::TemplateAssignmentRepository, Repository,
        RepositoryError, TransactionalRepository,
    },
};
use perroute_template::{render::TemplateRenderPlugin, repository::TemplateLookup};
use stack::Stacks;
use template::TemplateGenerator;

pub mod error;
pub mod stack;
pub mod template;

#[derive(Clone)]
pub struct Dispatcher<REPO, TRP, TR> {
    template_generator: TemplateGenerator<REPO, TRP, TR>,
    repository: REPO,
    stacks: Stacks<REPO>,
}

impl<REPO, TRP, TR> Dispatcher<REPO, TRP, TR>
where
    TRP: TemplateRenderPlugin,
    TR: TemplateLookup + Clone,
    REPO: Repository,
{
    pub fn new(
        template_generator: TemplateGenerator<REPO, TRP, TR>,
        repository: REPO,
        stacks: Stacks<REPO>,
    ) -> Self {
        Self {
            template_generator,
            repository,
            stacks,
        }
    }

    async fn retrieve_message(
        &self,
        event: &ApplicationEventData<MessageCreatedEvent>,
    ) -> Result<Option<Message>, RepositoryError> {
        match MessageRepository::find_message(
            &self.repository,
            MessageQuery::ById(event.entity_id()),
        )
        .await?
        {
            Some(message) => {
                if MessageStatus::Received != *message.status() {
                    return Ok(None);
                } else {
                    Ok(Some(message))
                }
            }
            None => Ok(None),
        }
    }

    async fn process(
        &self,
        message: &Message,
    ) -> Result<Vec<DispatcherLog>, DispatchError> {
        let stack = self.stacks.create(&message).await?;
        let template = self.template_generator.generate(&message).await?;
        let request = DispatchRequest::create(
            message.id(),
            message.recipient(),
            &template,
        )?;

        Ok(stack.dispatch(request).await)
    }

    pub async fn dispatch(
        self,
        event: ApplicationEventData<MessageCreatedEvent>,
    ) -> Result<(), DispatchError> {
        let mut message = match self.retrieve_message(&event).await? {
            Some(message) => message,
            None => return Ok(()),
        };

        let tx = self.repository.begin_transaction().await?;
        let logs = self.process(&message).await?;
        let logs =
            DispatcherLogRepository::save_all_dispatch_logs(&tx, logs).await?;

        message = message.set_status(if logs.iter().any(|l| *l.success()) {
            MessageStatus::Dispatched
        } else {
            MessageStatus::Failed
        });

        message = message.set_updated_at(Timestamp::now());

        MessageRepository::update(&tx, message).await?;

        tx.commit().await?;

        Ok(())
    }
}
pub fn create_dispatcher<REPO, TRP, TL>(
    repository: REPO,
    template_render_plugin: TRP,
    template_lookup: TL,
    plugin_repository: ProviderPluginRepository,
) -> Dispatcher<REPO, TRP, TL>
where
    REPO: Repository
        + BusinessUnitRepository
        + MessageTypeRepository
        + TemplateAssignmentRepository
        + RouteRepository
        + Clone,
    TRP: TemplateRenderPlugin,
    TL: TemplateLookup,
{
    Dispatcher {
        template_generator: TemplateGenerator::new(
            template_render_plugin,
            template_lookup,
            repository.clone(),
        ),
        repository: repository.clone(),
        stacks: Stacks::new(repository, plugin_repository),
    }
}
