use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
};
use bon::Builder;
use perroute_commons::{
    events::ChannelUpdatedEvent,
    types::{dispatch_type::DispatchType, id::Id, name::Name, Configuration, ProviderId},
};
use perroute_storage::{
    models::channel::Channel,
    repository::{
        business_units::{BusinessUnitQuery, BusinessUnitRepository},
        channels::ChannelRepository,
        TransactedRepository,
    },
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum CreateChannelCommandError {
    #[error("Business unit not found")]
    BusinessUnitNotFound,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct CreateChannelCommand {
    #[builder(default)]
    id: Id,
    business_unit_id: Id,
    name: Name,
    provider_id: ProviderId,
    dispatch_type: DispatchType,
    configuration: Configuration,
    enabled: bool,
}

impl Command for CreateChannelCommand {
    fn event_type(&self) -> perroute_commons::events::EventType {
        perroute_commons::events::EventType::ChannelCreated
    }

    fn entity_id(&self) -> &Id {
        &self.id
    }
}

pub struct CreateChannelCommandHandler;

impl CommandHandler for CreateChannelCommandHandler {
    type Command = CreateChannelCommand;
    type Output = Channel;
    type ApplicationEvent = ChannelUpdatedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        let exists_bu = BusinessUnitRepository::exists_business_unit(
            ctx.repository(),
            &BusinessUnitQuery::ById(cmd.inner().business_unit_id.clone()),
        )
        .await?;

        if !exists_bu {
            return Err(CreateChannelCommandError::BusinessUnitNotFound.into());
        }

        let channel = Channel::builder()
            .id(cmd.inner().id.clone())
            .business_unit_id(cmd.inner().business_unit_id.clone())
            .name(cmd.inner().name.clone())
            .provider_id(cmd.inner().provider_id.clone())
            .dispatch_type(cmd.inner().dispatch_type.clone())
            .configuration(cmd.inner().configuration.clone())
            .enabled(cmd.inner().enabled)
            .created_at(cmd.created_at().clone())
            .updated_at(cmd.created_at().clone())
            .build();

        let channel = ChannelRepository::save(ctx.repository(), channel).await?;

        //Ok(channel)
        todo!()
    }
}
