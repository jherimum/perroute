use error::ApiError;
use models::{ResourceModel, ResourceModelCollection};
use perroute_command_bus::CommandBus;
use perroute_query_bus::QueryBus;

pub mod error;
pub mod models;
pub mod modules;

pub type RestServiceResult<T> = Result<T, ApiError>;
pub type ResourceModelResult<T> = RestServiceResult<ResourceModel<T>>;
pub type ResourceModelCollectionResult<T> = RestServiceResult<ResourceModelCollection<T>>;
pub type EmptyResourceModelResult = RestServiceResult<()>;

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
