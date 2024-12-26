use error::ApiError;
use models::resource::{ResourceModel, ResourceModelCollection};

pub mod error;
pub mod models;
pub mod modules;
pub mod service;

pub type RestServiceResult<T> = Result<T, ApiError>;
pub type ResourceModelResult<T> = RestServiceResult<ResourceModel<T>>;
pub type MaybeResourceModelResult<T> =
    RestServiceResult<Option<ResourceModel<T>>>;
pub type ResourceModelCollectionResult<T> =
    RestServiceResult<ResourceModelCollection<T>>;
pub type EmptyResourceModelResult = RestServiceResult<()>;
