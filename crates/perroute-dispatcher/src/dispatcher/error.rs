use std::error::Error;

use perroute_commons::types::{id::Id, MessageStatus, ProviderId};
use perroute_connectors::{PluginDispatchError, ProviderPluginError};
use perroute_storage::{
    active_record::ActiveRecordError, repository::RepositoryError,
};
use perroute_template::{
    error::TemplateError, render::RenderError,
    repository::TemplateRepositoryError,
};

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("{0}")]
    ActiveRecordError(#[from] ActiveRecordError),

    #[error("Message not found: {0}")]
    MessageNotFound(Id),

    #[error("Invalid message status: {0:?}")]
    InvalidMessageStatus(Id, MessageStatus),

    #[error("{0}")]
    TemplaterError(#[from] TemplateError),

    #[error("{0}")]
    PluginDispatchError(#[from] PluginDispatchError),

    #[error("No template assignment eligible")]
    NoTemplateAssignmentEligible,

    #[error("No route Eligible")]
    NoRouteEligible,

    #[error("Provider plugin not found: {0}")]
    ProviderPluginNotFound(ProviderId),

    #[error("{0}")]
    ProviderPluginError(#[from] ProviderPluginError),

    #[error("{0}")]
    PluginNotFound(ProviderId),

    #[error("{0}")]
    RenderError(#[from] RenderError),

    #[error("{0}")]
    TemplateRepositoryError(#[from] TemplateRepositoryError),

    #[error("{0}")]
    RepositoryError(#[from] RepositoryError),

    #[error("{0}")]
    UnexpectedError(#[from] Box<dyn Error + Send + Sync>),

    #[error("{0}")]
    TemplateNotFound(String),
}
