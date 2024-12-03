use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command, CommandBusError,
};
use perroute_commons::{
    events::RouteCreatedEvent,
    types::{id::Id, priority::Priority, Configuration},
};
use perroute_storage::{
    models::route::Route,
    repository::{
        business_units::{BusinessUnitQuery, BusinessUnitRepository},
        channels::{ChannelQuery, ChannelRepository},
        message_types::{MessageTypeQuery, MessageTypeRepository},
        routes::RouteRepository,
        TransactedRepository,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum CreateRouteCommandError {
    #[error("message type not found")]
    MessageTypeNotFound,

    #[error("channel type not found")]
    ChannelTypeNotFound,

    #[error("business unit not found")]
    BusinessUnitNotFound,
}

impl_command!(CreateRouteCommand, {
    route_id: Id,
    business_id: Id,
    channel_id: Id,
    message_type_id: Id,
    configuration: Configuration,
    priority: Priority,
    enabled: bool,
});

pub struct CreateRouteCommandHandler;

impl CommandHandler for CreateRouteCommandHandler {
    type Command = CreateRouteCommand;
    type Output = Route;
    type ApplicationEvent = RouteCreatedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        validate(cmd.inner(), ctx).await?;

        let route = Route::builder()
            .id(cmd.inner().route_id.clone())
            .channel_id(cmd.inner().channel_id.clone())
            .message_type_id(cmd.inner().message_type_id.clone())
            .configuration(cmd.inner().configuration.clone())
            .priority(cmd.inner().priority.clone())
            .enabled(cmd.inner().enabled)
            .created_at(cmd.created_at().clone())
            .updated_at(cmd.created_at().clone())
            .build();

        let route = RouteRepository::save(ctx.repository(), route.clone()).await?;

        //Ok(route.clone())
        todo!()
    }
}

async fn validate<R: TransactedRepository>(
    cmd: &CreateRouteCommand,
    ctx: &CommandBusContext<'_, R>,
) -> Result<(), CommandBusError> {
    let exists = BusinessUnitRepository::exists_business_unit(
        ctx.repository(),
        &BusinessUnitQuery::ById(cmd.business_id.clone()),
    )
    .await?;

    if !exists {
        return Err(CreateRouteCommandError::BusinessUnitNotFound.into());
    }

    let exists = MessageTypeRepository::exists_message_type(
        ctx.repository(),
        &MessageTypeQuery::ById(cmd.message_type_id.clone()),
    )
    .await?;

    if !exists {
        return Err(CreateRouteCommandError::MessageTypeNotFound.into());
    }

    let channel =
        ChannelRepository::find(ctx.repository(), &ChannelQuery::ById(&cmd.channel_id)).await?;

    if let Some(channel) = channel {
        if *channel.business_unit_id() == cmd.business_id {
            return Ok(());
        }
    }

    Err(CreateRouteCommandError::ChannelTypeNotFound.into())
}
