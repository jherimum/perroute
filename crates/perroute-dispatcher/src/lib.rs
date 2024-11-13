use perroute_commons::{
    template::TemplateRender,
    types::{
        dispatch_type::DispatchType, id::Id, recipient::EmailRecipient, template::EmailTemplate,
        MessageStatus,
    },
};
use perroute_connectors::{DispatcherTrait, ProviderPluginRepository, Request};
use perroute_storage::{
    models::{
        business_unit::BusinessUnit, channel::Channel, message::Message, route::Route,
        template_assignment::TemplateAssignment,
    },
    repository::{
        business_units::BusinessUnitRepository,
        channels::{ChannelQuery, ChannelRepository},
        message::{MessageQuery, MessageRepository},
        routes::{ActiveByBusinessUnitAndDispatchTypeQuery, RouteQuery, RouteRepository},
        Repository,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum DispatcherError {
    #[error("Repository error: {0}")]
    RepositoryError(#[from] perroute_storage::repository::Error),
}

pub struct Dispatcher<R, PR, TR> {
    repository: R,
    plugin_repository: PR,
    template_render: TR,
}

impl<R, PR, TR> Dispatcher<R, PR, TR> {
    pub fn new(repository: R, plugin_repository: PR, template_render: TR) -> Self {
        Dispatcher {
            repository,
            plugin_repository,
            template_render,
        }
    }
}

impl<R: Repository, PR: ProviderPluginRepository, TR: TemplateRender> Dispatcher<R, PR, TR> {
    pub async fn dispatch(&self, message_id: &Id) -> Result<(), DispatcherError> {
        let message =
            MessageRepository::query(&self.repository, &MessageQuery::ById(message_id)).await?;

        let message = match message {
            Some(message) => {
                if *message.status() == MessageStatus::Received {
                    log::warn!(
                        "Message with id {} is not elegible to be dispatched. Actual status is {}.",
                        message_id,
                        message.status()
                    );
                    return Ok(());
                } else {
                    message
                }
            }
            None => {
                log::warn!("Message with id {} not found", message_id);
                return Ok(());
            }
        };

        let business_unit =
            BusinessUnitRepository::get(&self.repository, message.business_unit_id()).await?;

        let mut routes = RouteRepository::query(
            &self.repository,
            &RouteQuery::ActiveByBusinessUnitAndDispatchType(
                &ActiveByBusinessUnitAndDispatchTypeQuery {
                    business_unit_id: message.business_unit_id(),
                    message_type_id: message.message_type_id(),
                    dispatch_type: message.dispatch_type(),
                },
            ),
        )
        .await?;

        routes.sort_by(|r1, r2| r1.priority().cmp(r2.priority()));

        if routes.len() == 0 {
            log::warn!(
                "No routes found for message with id {} and dispatch type {}",
                message.id(),
                message.dispatch_type()
            );
            return Ok(());
        }

        let template = match select_template() {
            Some(template) => template,
            None => {
                log::warn!(
                    "No template found for message with id {} and dispatch type {}",
                    message.id(),
                    message.dispatch_type()
                );
                return Ok(());
            }
        };

        for route in routes {
            let channel =
                ChannelRepository::find(&self.repository, &ChannelQuery::ById(route.channel_id()))
                    .await?;

            let channel = match channel {
                Some(channel) => {
                    if !*channel.enabled() {
                        log::warn!("Channel with id {} is disabled.", route.channel_id());
                        continue;
                    }
                    channel
                }
                None => {
                    log::warn!("Channel with id {} not found.", route.channel_id(),);
                    continue;
                }
            };

            let configuration = channel.configuration().merge(route.configuration());

            let plugin = match self.plugin_repository.get(channel.provider_id()) {
                Some(plugin) => plugin,
                None => {
                    log::warn!("Provider with id {} not found.", channel.provider_id());
                    continue;
                }
            };

            let r = EmailRecipient {};
            let t = EmailTemplate {};

            let result = match message.dispatch_type() {
                DispatchType::Email => {
                    let request = Request::email(r, t);
                    let dispatcher = plugin.email_dispatcher(configuration).unwrap();
                    dispatcher.dispatch(request).await
                }
                DispatchType::Sms => todo!(),
                DispatchType::Push => todo!(),
            };

            // let req = build_request(&business_unit, &channel, &route, &template, &message);
            // let dispatch_result = plugin.dispatch(&req, &config).await;
        }

        Ok(())
    }
}

fn select_template() -> Option<TemplateAssignment> {
    // let templates = TemplateAssignmentRepository::query(
    //     &self.repository,
    //     &TemplateAssignmentQuery::ForDispatch(QueryForDispatch {
    //         business_unit_id: message.business_unit_id(),
    //         message_type_id: message.message_type_id(),
    //         dispatch_type: message.dispatch_type(),
    //         date_reference: &Timestamp::now(),
    //     }),
    // )
    // .await?;

    todo!()
}

// fn build_request<'r>(
//     business_unit: &BusinessUnit,
//     channel: &Channel,
//     route: &Route,
//     template_assignment: &TemplateAssignment,
//     message: &Message,
// ) -> DispatchRequest<'r> {
//     match message.dispatch_type() {
//         perroute_commons::types::dispatch_type::DispatchType::Email => todo!(),
//         perroute_commons::types::dispatch_type::DispatchType::Sms => todo!(),
//         perroute_commons::types::dispatch_type::DispatchType::Push => todo!(),
//     }
// }
