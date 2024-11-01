use perroute_command_bus::CommandBus;
use perroute_query_bus::QueryBus;

use crate::rest::RestService;

pub trait RouteRestService {}

impl<CB: CommandBus, QB: QueryBus> RouteRestService for RestService<CB, QB> {}
