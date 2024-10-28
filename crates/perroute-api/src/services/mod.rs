use business_units::BusinessUnitRestService;
use perroute_command_bus::CommandBus;
pub mod business_units;

pub trait RestService: BusinessUnitRestService {}

#[derive(Clone)]
pub struct DefaultRestService<Cb> {
    command_bus: Cb,
}

impl<Cb: CommandBus> DefaultRestService<Cb> {
    pub fn new(command_bus: Cb) -> Self {
        Self { command_bus }
    }
}

impl<Cb: CommandBus> RestService for DefaultRestService<Cb> {}

impl<Cb: CommandBus> BusinessUnitRestService for DefaultRestService<Cb> {}
