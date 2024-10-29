use error::ApiError;
use models::{ResourceModel, ResourceModelCollection};

pub mod error;
pub mod models;
pub mod routes;
pub mod services;

pub type RestServiceResult<T> = Result<T, ApiError>;
pub type ResourceModelResult<T> = RestServiceResult<ResourceModel<T>>;
pub type ResourceModelCollectionResult<T> = RestServiceResult<ResourceModelCollection<T>>;
pub type EmptyResourceModelResult = RestServiceResult<()>;
