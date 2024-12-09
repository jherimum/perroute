use crate::publisher::{Publisher, PublisherOutput, PublisherResult};
use aws_sdk_sns::{
    config::http::HttpResponse,
    error::{BuildError, SdkError},
    operation::publish_batch::{PublishBatchError, PublishBatchOutput},
    types::{BatchResultErrorEntry, MessageAttributeValue, PublishBatchRequestEntry},
};
use perroute_commons::{
    events::{ApplicationEventData, Event},
    types::id::Id,
};
use serde::Serialize;
use std::collections::HashMap;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum SnsPublisherError {
    #[error("Failed to publish events: {0:?}")]
    SdkError(#[from] SdkError<PublishBatchError, HttpResponse>),

    #[error("Failed to build entry: {0}")]
    PublishBatchRequestEntryBuilderError(#[from] BuildError),

    #[error("Failed to serialize event: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Failed to publish batch: {0}")]
    PublishBatchError(#[from] PublishBatchError),

    #[error("Failed to build batch entries: {0}")]
    EntryBatchError(#[from] SqsBatchError),
}

pub struct SnsPublisher {
    sns_client: aws_sdk_sns::Client,
    topic_arn: String,
}

impl SnsPublisher {
    pub fn new(sns_client: aws_sdk_sns::Client, topic_arn: String) -> Self {
        Self {
            sns_client,
            topic_arn,
        }
    }

    async fn publish_to_sns(
        &self,
        events: &BatchEvents,
    ) -> Result<PublishBatchOutput, SnsPublisherError> {
        Ok(self
            .sns_client
            .publish_batch()
            .topic_arn(&self.topic_arn)
            .set_publish_batch_request_entries(Some(TryInto::try_into(events)?))
            .send()
            .await
            .tap_err(|e| log::error!("Failed to publish events: {e}"))?)
    }
}

impl Publisher for SnsPublisher {
    async fn publish(&self, events: Vec<Event>) -> PublisherResult {
        if events.is_empty() {
            return Ok(PublisherOutput::new());
        }

        let events = BatchEvents::new(events);
        let output = self
            .publish_to_sns(&events)
            .await
            .tap_err(|e| log::error!("Failed to publish events: {e}"))?;

        Ok(events.to_output(output))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("SqsBatchError {{ code: {code}, message: {message:?}, sender_fault: {sender_fault} }}")]
pub struct SqsBatchError {
    code: String,
    message: Option<String>,
    sender_fault: bool,
}

impl From<&BatchResultErrorEntry> for SqsBatchError {
    fn from(error: &BatchResultErrorEntry) -> Self {
        Self {
            code: error.code().to_string(),
            message: error.message().map(ToString::to_string),
            sender_fault: error.sender_fault(),
        }
    }
}

struct BatchEvents {
    events: HashMap<Id, Event>,
}

impl BatchEvents {
    fn new(events: Vec<Event>) -> Self {
        Self {
            events: events.into_iter().map(|e| (e.id().to_owned(), e)).collect(),
        }
    }

    fn find_event(&mut self, id: &str) -> Option<Event> {
        self.events.remove(&Id::from(id))
    }

    fn to_output(mut self, output: PublishBatchOutput) -> PublisherOutput {
        let mut publisher_output = PublisherOutput::new();

        for success in output.successful() {
            if let Some(success_id) = success.id() {
                if let Some(e) = self.find_event(success_id) {
                    publisher_output.push_success(e);
                }
            }
        }

        for failed in output.failed() {
            if let Some(event) = self.find_event(failed.id()) {
                publisher_output.push_failed(
                    event,
                    SnsPublisherError::from(SqsBatchError::from(failed)).into(),
                );
            }
        }

        publisher_output
    }
}

impl TryFrom<&BatchEvents> for Vec<PublishBatchRequestEntry> {
    type Error = SnsPublisherError;

    fn try_from(value: &BatchEvents) -> Result<Self, Self::Error> {
        let entries = value
            .events
            .values()
            .map(to_entry)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }
}

fn to_entry(event: &Event) -> Result<PublishBatchRequestEntry, SnsPublisherError> {
    match event {
        Event::BusinessUnitCreated(event_data) => from_event_data(event_data),
        Event::BusinessUnitUpdated(event_data) => from_event_data(event_data),
        Event::BusinessUnitDeleted(event_data) => from_event_data(event_data),
        Event::ChannelCreated(event_data) => from_event_data(event_data),
        Event::ChannelUpdated(event_data) => from_event_data(event_data),
        Event::ChannelDeleted(event_data) => from_event_data(event_data),
        Event::MessageTypeCreated(event_data) => from_event_data(event_data),
        Event::MessageTypeUpdated(event_data) => from_event_data(event_data),
        Event::MessageTypeDeleted(event_data) => from_event_data(event_data),
        Event::RouteCreated(event_data) => from_event_data(event_data),
        Event::RouteUpdated(event_data) => from_event_data(event_data),
        Event::RouteDeleted(event_data) => from_event_data(event_data),
        Event::MessageCreated(event_data) => from_event_data(event_data),
        Event::TemplateAssignmentCreated(event_data) => from_event_data(event_data),
        Event::TemplateAssignmentUpdated(event_data) => from_event_data(event_data),
        Event::TemplateAssignmentDeleted(event_data) => from_event_data(event_data),
    }
}

fn from_event_data<P: Serialize>(
    event_data: &ApplicationEventData<P>,
) -> Result<PublishBatchRequestEntry, SnsPublisherError> {
    Ok(PublishBatchRequestEntry::builder()
        .id(event_data.id())
        .message_group_id(event_data.entity_id())
        .message_deduplication_id(event_data.id())
        .message(serde_json::to_string(event_data)?)
        .message_attributes(
            "event_type",
            MessageAttributeValue::builder()
                .data_type("String")
                .string_value(event_data.event_type().to_string())
                .build()?,
        )
        .build()?)
}
