use crate::{
    bus::{Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult},
    CommandBusError,
};
use bon::{builder, Builder};
use perroute_commons::{
    commands::CommandType,
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
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum CreateRouteCommandError {
    #[error("message type not found")]
    MessageTypeNotFound,

    #[error("channel type not found")]
    ChannelTypeNotFound,

    #[error("business unit not found")]
    BusinessUnitNotFound,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct CreateRouteCommand {
    #[builder(default)]
    id: Id,
    business_id: Id,
    channel_id: Id,
    message_type_id: Id,
    configuration: Configuration,
    priority: Priority,
    enabled: bool,
}

impl Command for CreateRouteCommand {
    fn command_type(&self) -> CommandType {
        CommandType::CreateRoute
    }

    fn to_event<R: TransactedRepository>(
        &self,
        ctx: &CommandBusContext<'_, R>,
    ) -> perroute_commons::events::Event {
        todo!()
    }
}

pub struct CreateRouteCommandHandler;

impl CommandHandler for CreateRouteCommandHandler {
    type Command = CreateRouteCommand;
    type Output = Route;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        validate(cmd, &ctx).await?;

        let route = Route::builder()
            .id(cmd.id.clone())
            .channel_id(cmd.channel_id.clone())
            .message_type_id(cmd.message_type_id.clone())
            .configuration(cmd.configuration.clone())
            .priority(cmd.priority.clone())
            .enabled(cmd.enabled)
            .created_at(ctx.created_at().clone())
            .updated_at(ctx.created_at().clone())
            .build();

        let route = RouteRepository::save(ctx.repository(), route.clone()).await?;

        CommandHandlerOutput::new(route.clone()).ok()
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
