use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command, CommandBusError,
};
use perroute_commons::{
    events::ChannelCreatedEvent,
    types::{
        dispatch_type::DispatchType, id::Id, name::Name, Configuration,
        ProviderId,
    },
};
use perroute_connectors::ProviderPluginRepository;
use perroute_storage::{
    active_record::{
        business_unit::BusinessUnitQuery, channel::CreateChannel,
        datasource::Connection, ActiveRecord,
    },
    models::{business_unit::BusinessUnit, channel::Channel},
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateChannelCommandError {
    #[error("Business unit not found")]
    BusinessUnitNotFound,

    #[error("Plugin {0} not found")]
    PluginNotFound(ProviderId),
}

impl_command!(CreateChannelCommand, {
    channel_id: Id,
    business_unit_id: Id,
    name: Name,
    provider_id: ProviderId,
    dispatch_type: DispatchType,
    configuration: Configuration,
    enabled: bool,

});

pub struct CreateChannelCommandHandler;

impl CreateChannelCommandHandler {
    fn check_plugin(
        plugin_repository: &ProviderPluginRepository,
        id: &ProviderId,
    ) -> CommandHandlerResult<()> {
        match plugin_repository.get(id) {
            Some(_) => Ok(()),
            None => {
                Err(CreateChannelCommandError::PluginNotFound(id.clone())
                    .into())
            }
        }
    }

    async fn check_bu_existence<C: AsRef<Connection>>(
        conn: C,
        id: &Id,
    ) -> CommandHandlerResult<()> {
        match BusinessUnit::exists(conn, BusinessUnitQuery::ById(id))
            .await
            .tap_err(|e| log::error!("Failed to check of bu exists: {e}"))?
        {
            true => Ok(()),
            false => {
                Err(CreateChannelCommandError::BusinessUnitNotFound.into())
            }
        }
    }
}

impl CommandHandler for CreateChannelCommandHandler {
    type Command = CreateChannelCommand;
    type Output = Channel;
    type E = ChannelCreatedEvent;

    fn into_event(
        command: &Self::Command,
        output: &Self::Output,
    ) -> Option<ChannelCreatedEvent> {
        Some(
            ChannelCreatedEvent::builder()
                .id(output.id())
                .business_unit_id(&command.business_unit_id)
                .name(&command.name)
                .provider_id(&command.provider_id)
                .dispatch_type(command.dispatch_type)
                .configuration(&command.configuration)
                .enabled(command.enabled)
                .build(),
        )
    }

    async fn handle<C: AsRef<Connection>>(
        &self,
        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> CommandHandlerResult<Self::Output> {
        Self::check_bu_existence(
            ctx.datasource(),
            &ctx.command().business_unit_id,
        )
        .await?;

        Self::check_plugin(
            ctx.plugin_repository(),
            &ctx.command().provider_id,
        )?;

        Channel::create(ctx.datasource(), ctx.into())
            .await
            .tap_err(|e| log::error!("Failed to save channel: {e}"))
            .map_err(CommandBusError::from)
    }
}

impl<C: AsRef<Connection>> From<&CommandBusContext<'_, C, CreateChannelCommand>>
    for CreateChannel
{
    fn from(ctx: &CommandBusContext<'_, C, CreateChannelCommand>) -> Self {
        CreateChannel::builder()
            .business_unit_id(&ctx.command().business_unit_id)
            .configuration(&ctx.command().configuration)
            .dispatch_type(ctx.command().dispatch_type)
            .enabled(ctx.command().enabled)
            .name(&ctx.command().name)
            .provider_id(&ctx.command().provider_id)
            .timestamp(ctx.timestamp())
            .build()
    }
}
