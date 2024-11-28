use crate::sns::SnsPublisherError;
use perroute_storage::models::event::DbEvent;
use std::future::Future;

pub type PublisherResult = Result<(), PublisherError>;

#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    #[error("SNS publisher error: {0}")]
    SnsPublisherError(#[from] SnsPublisherError),
}

pub trait Publisher {
    fn publish<'e>(&self, events: &'e Vec<DbEvent>) -> impl Future<Output = PublisherResult>;
}
