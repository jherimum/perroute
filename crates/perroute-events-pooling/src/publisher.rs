use perroute_commons::events::Event;

use crate::sns::SnsPublisherError;
use std::future::Future;

pub type PublisherResult = Result<(), PublisherError>;

#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    #[error("SNS publisher error: {0}")]
    SnsPublisherError(#[from] SnsPublisherError),
}

pub trait Publisher {
    fn publish(&self, events: &[Event]) -> impl Future<Output = PublisherResult>;
}
