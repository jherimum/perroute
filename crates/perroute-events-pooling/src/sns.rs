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
        events: &BatchEvents<'_>,
    ) -> Result<PublishBatchOutput, SnsPublisherError> {
        let entries: Vec<_> = TryFrom::try_from(events)?;

        Ok(self
            .sns_client
            .publish_batch()
            .topic_arn(&self.topic_arn)
            .set_publish_batch_request_entries(Some(entries))
            .send()
            .await
            .map_err(SnsPublisherError::from)?)
    }
}

impl Publisher for SnsPublisher {
    async fn publish<'e>(&self, events: &'e Vec<Event>) -> PublisherResult<'e> {
        let events = BatchEvents::new(events);
        let output = self.publish_to_sns(&events).await?;
        Ok(events.to_output(output))
    }
}

fn to_entry(event: &Event) -> Result<PublishBatchRequestEntry, SnsPublisherError> {
    match event {
        Event::BusinessUnitCreated(event_data) => to_entry_e(event_data),
        Event::BusinessUnitUpdated(event_data) => to_entry_e(event_data),
        Event::BusinessUnitDeleted(event_data) => to_entry_e(event_data),
        Event::ChannelCreated(event_data) => to_entry_e(event_data),
        Event::ChannelUpdated(event_data) => to_entry_e(event_data),
        Event::ChannelDeleted(event_data) => to_entry_e(event_data),
        Event::MessageTypeCreated(event_data) => to_entry_e(event_data),
        Event::MessageTypeUpdated(event_data) => to_entry_e(event_data),
        Event::MessageTypeDeleted(event_data) => to_entry_e(event_data),
        Event::RouteCreated(event_data) => to_entry_e(event_data),
        Event::RouteUpdated(event_data) => to_entry_e(event_data),
        Event::RouteDeleted(event_data) => to_entry_e(event_data),
        Event::MessageCreated(event_data) => to_entry_e(event_data),
        Event::TemplateAssignmentCreated(event_data) => to_entry_e(event_data),
        Event::TemplateAssignmentUpdated(event_data) => to_entry_e(event_data),
        Event::TemplateAssignmentDeleted(event_data) => to_entry_e(event_data),
    }
}

fn to_entry_e<P: Serialize>(
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
                .build()
                .unwrap(),
        )
        .build()?)
}

#[derive(Debug, thiserror::Error)]
#[error("SqsBatchError {{ code: {code}, message: {message:?}, sender_fault: {sender_fault} }}")]
pub struct SqsBatchError {
    pub code: String,
    pub message: Option<String>,
    pub sender_fault: bool,
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

struct BatchEvents<'e> {
    events: &'e Vec<Event>,
}

impl<'e> BatchEvents<'e> {
    fn new(events: &'e Vec<Event>) -> Self {
        Self { events }
    }

    fn find_event(&self, id: &str) -> Option<&'e Event> {
        self.events.iter().find(|e| *e.id() == Id::from(id))
    }

    fn to_output(&self, output: PublishBatchOutput) -> PublisherOutput<'e> {
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

impl<'e> TryFrom<&BatchEvents<'e>> for Vec<PublishBatchRequestEntry> {
    type Error = SnsPublisherError;

    fn try_from(value: &BatchEvents) -> Result<Self, Self::Error> {
        let entries = value
            .events
            .iter()
            .map(to_entry)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }
}
