pub mod business_units;

use business_units::BusinessUnitRestService;
use perroute_command_bus::CommandBus;
use perroute_query_bus::QueryBus;

pub trait RestService: BusinessUnitRestService {}

#[derive(Clone)]
pub struct DefaultRestService<CB, QB> {
    command_bus: CB,
    query_bus: QB,
}

impl<CB: CommandBus, QB: QueryBus> DefaultRestService<CB, QB> {
    pub fn new(command_bus: CB, query_bus: QB) -> Self {
        Self {
            command_bus,
            query_bus,
        }
    }
}

impl<CB: CommandBus, QB: QueryBus> RestService for DefaultRestService<CB, QB> {}
