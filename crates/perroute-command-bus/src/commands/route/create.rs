use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::{builder, Builder};
use perroute_commons::{
    commands::CommandType,
    events::Event,
    types::{id::Id, priority::Priority, Configuration, Timestamp},
};
use perroute_storage::{
    models::route::Route,
    repository::{
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
}

pub struct CreateRouteCommandHandler;

impl CommandHandler for CreateRouteCommandHandler {
    type Command = CreateRouteCommand;
    type Output = Route;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let exists = MessageTypeRepository::exists_message_type(
            ctx.repository(),
            &MessageTypeQuery::ById(cmd.message_type_id.clone()),
        )
        .await?;

        if !exists {
            return Err(CreateRouteCommandError::MessageTypeNotFound.into());
        }

        let exists = ChannelRepository::exists_channel(
            ctx.repository(),
            &ChannelQuery::ById(cmd.channel_id.clone()),
        )
        .await?;

        if !exists {
            return Err(CreateRouteCommandError::MessageTypeNotFound.into());
        }

        let route = Route::builder()
            .id(cmd.id.clone())
            .channel_id(cmd.channel_id.clone())
            .message_type_id(cmd.message_type_id.clone())
            .configuration(cmd.configuration.clone())
            .priority(cmd.priority.clone())
            .enabled(cmd.enabled)
            .created_at(Timestamp::now())
            .updated_at(Timestamp::now())
            .build();

        let route = RouteRepository::save(ctx.repository(), route.clone()).await?;

        CommandHandlerOutput::new(route.clone())
            .with_event(Event::RouteCreated(route.id().clone()))
            .ok()
    }
}
