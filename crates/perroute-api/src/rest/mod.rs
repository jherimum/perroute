use error::ApiError;
use models::{ResourceModel, ResourceModelCollection};
use perroute_command_bus::CommandBus;
use perroute_query_bus::QueryBus;

pub mod error;
pub mod models;
pub mod modules;
pub mod service;

pub type RestServiceResult<T> = Result<T, ApiError>;
pub type ResourceModelResult<T> = RestServiceResult<ResourceModel<T>>;
pub type ResourceModelCollectionResult<T> = RestServiceResult<ResourceModelCollection<T>>;
pub type EmptyResourceModelResult = RestServiceResult<()>;
