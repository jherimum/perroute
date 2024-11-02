use perroute_command_bus::CommandBus;
use perroute_query_bus::QueryBus;

#[derive(Clone)]
pub struct RestService<CB, QB> {
    command_bus: CB,
    query_bus: QB,
}

impl<CB: CommandBus, QB: QueryBus> RestService<CB, QB> {
    pub fn new(command_bus: CB, query_bus: QB) -> Self {
        Self {
            command_bus,
            query_bus,
        }
    }

    pub fn command_bus(&self) -> &CB {
        &self.command_bus
    }

    pub fn query_bus(&self) -> &QB {
        &self.query_bus
    }
}
