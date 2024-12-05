use crate::sns::SnsPublisherError;
use perroute_commons::events::Event;
use std::future::Future;

pub type PublisherResult = Result<PublisherOutput, PublisherError>;

#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    #[error("SNS publisher error: {0}")]
    SnsPublisherError(#[from] SnsPublisherError),
}

pub trait Publisher {
    fn publish(&self, events: Vec<Event>) -> impl Future<Output = PublisherResult>;
}

#[derive(Debug, Default)]
pub struct PublisherOutput {
    success: Vec<Event>,
    failed: Vec<(Event, PublisherError)>,
}

impl PublisherOutput {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_success(&mut self, event: Event) {
        self.success.push(event);
    }

    pub fn push_failed(&mut self, event: Event, error: PublisherError) {
        self.failed.push((event, error));
    }
}
