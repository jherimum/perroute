use crate::sns::SnsPublisherError;
use perroute_commons::new_events::NewEvent;
use std::future::Future;

pub type PublisherResult = Result<(), PublisherError>;

#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    #[error("SNS publisher error: {0}")]
    SnsPublisherError(#[from] SnsPublisherError),
}

pub trait Publisher {
    fn publish(&self, events: &[NewEvent]) -> impl Future<Output = PublisherResult>;
}
