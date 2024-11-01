use perroute_command_bus::CommandBus;
use perroute_query_bus::QueryBus;

use crate::rest::RestService;




pub trait ChannelRestService{}


impl<CB: CommandBus, QB: QueryBus> ChannelRestService for RestService<CB, QB> {
    
}