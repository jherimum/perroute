mod bus;
pub mod commands;
mod error;

#[macro_export]
macro_rules! impl_command {
    ($name:ident, { $( $field_name:ident : $field_type:ty ),* $(,)? }) => {

            #[derive(Debug, Clone, bon::Builder)]
            pub struct $name {
                $(
                    #[builder(into)]
                    pub $field_name: $field_type
                ),*
            }

            impl Command for $name {}

    };
}

pub use bus::CommandBus;
use bus::DefaultCommandBus;
use commands::{
    business_unit::{
        create::CreateBusinessUnitCommandHandler,
        delete::DeleteBusinessUnitCommandHandler,
        update::UpdateBusinessUnitCommandHandler,
    },
    channel::{
        create::CreateChannelCommandHandler,
        delete::DeleteChannelCommandHandler,
        update::UpdateChannelCommandHandler,
    },
    message::create::CreateMessageCommandHandler,
    message_type::{
        create::CreateMessageTypeCommandHandler,
        update::UpdateMessageTypeCommandHandler,
    },
    route::{
        create::CreateRouteCommandHandler, delete::DeleteRouteCommandHandler,
        update::UpdateRouteCommandHandler,
    },
    template_assignment::{
        create::CreateTemplateAssignmentCommandHandler,
        delete::DeleteTemplateAssignmentCommandHandler,
        update::UpdateTemplateAssignmentCommandHandler,
    },
};
pub use error::CommandBusError;
use perroute_connectors::ProviderPluginRepository;
use perroute_storage::active_record::datasource::{
    DataSource, NonTransactionalDataSource,
};
pub type CommandBusResult<T> = Result<T, CommandBusError>;

pub fn create_command_bus(
    datasource: DataSource<NonTransactionalDataSource>,
    plugin_repository: ProviderPluginRepository,
) -> impl CommandBus + Clone {
    DefaultCommandBus::new(datasource, plugin_repository)
        .register(CreateBusinessUnitCommandHandler)
        .register(DeleteBusinessUnitCommandHandler)
        .register(UpdateBusinessUnitCommandHandler)
        .register(CreateMessageTypeCommandHandler)
        .register(UpdateMessageTypeCommandHandler)
        .register(DeleteBusinessUnitCommandHandler)
        .register(CreateRouteCommandHandler)
        .register(UpdateRouteCommandHandler)
        .register(DeleteRouteCommandHandler)
        .register(CreateChannelCommandHandler)
        .register(UpdateChannelCommandHandler)
        .register(DeleteChannelCommandHandler)
        .register(CreateTemplateAssignmentCommandHandler)
        .register(UpdateTemplateAssignmentCommandHandler)
        .register(DeleteTemplateAssignmentCommandHandler)
        .register(CreateMessageCommandHandler)
}
