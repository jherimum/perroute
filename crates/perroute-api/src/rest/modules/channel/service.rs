use crate::rest::RestService;
use perroute_command_bus::CommandBus;
use perroute_query_bus::QueryBus;

pub trait ChannelRestService {}

impl<CB: CommandBus, QB: QueryBus> ChannelRestService for RestService<CB, QB> {}
