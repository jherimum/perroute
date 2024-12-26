use crate::sns::SnsPublisherError;
use perroute_commons::events::Event;
use std::future::Future;

pub type PublisherResult<'e> = Result<PublisherOutput<'e>, PublisherError>;

#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    #[error("SNS publisher error: {0}")]
    SnsPublisherError(#[from] SnsPublisherError),
}

pub trait Publisher {
    fn publish<'e>(
        &self,
        events: Vec<&'e Event>,
    ) -> impl Future<Output = PublisherResult<'e>>;
}

#[derive(Debug, Default)]
pub struct PublisherOutput<'e> {
    success: Vec<&'e Event>,
    failed: Vec<(&'e Event, PublisherError)>,
}

impl<'e> PublisherOutput<'e> {
    pub fn success(&self) -> &Vec<&'e Event> {
        &self.success
    }

    pub fn failed(&self) -> &Vec<(&'e Event, PublisherError)> {
        &self.failed
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_success(&mut self, event: &'e Event) {
        self.success.push(event);
    }

    pub fn push_failed(&mut self, event: &'e Event, error: PublisherError) {
        self.failed.push((event, error));
    }
}
