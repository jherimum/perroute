mod bus;
pub mod commands;
mod error;

pub use bus::CommandBus;
use bus::DefaultCommandBus;
use commands::{
    business_unit::{
        create::CreateBusinessUnitCommandHandler, delete::DeleteBusinessUnitCommandHandler,
        update::UpdateBusinessUnitCommandHandler,
    },
    channel::{
        create::CreateChannelCommandHandler, delete::DeleteChannelCommandHandler,
        update::UpdateChannelCommandHandler,
    },
    message::create::CreateMessageCommandHandler,
    message_type::{
        create::CreateMessageTypeCommandHandler, update::UpdateMessageTypeCommandHandler,
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
use perroute_storage::repository::Repository;
pub type CommandBusResult<T> = Result<T, CommandBusError>;

pub fn create_command_bus<R: Repository + Clone>(repository: R) -> impl CommandBus + Clone {
    DefaultCommandBus::new(repository)
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
