use crate::publisher::{Publisher, PublisherResult};
use aws_sdk_sns::{
    config::http::HttpResponse,
    error::{BuildError, SdkError},
    operation::publish_batch::{PublishBatchError, PublishBatchOutput},
    types::{BatchResultErrorEntry, MessageAttributeValue, PublishBatchRequestEntry},
};
use perroute_commons::events::{Event, ApplicationEventData};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum SnsPublisherError {
    #[error("Failed to publish events: {0:?}")]
    SdkError(#[from] SdkError<PublishBatchError, HttpResponse>),

    #[error("Failed to build entry: {0}")]
    PublishBatchRequestEntryBuilderError(#[from] BuildError),

    #[error("Failed to serialize event: {0}")]
    SerializationError(#[from] serde_json::Error),
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
}

impl Publisher for SnsPublisher {
    async fn publish(&self, events: &[Event]) -> PublisherResult {
        let entries = match events.iter().map(to_entry).collect::<Result<Vec<_>, _>>() {
            Ok(entries) if entries.is_empty() => {
                log::info!("No events to publish");
                return Ok(());
            }
            Ok(entries) => entries,
            Err(e) => {
                log::error!("Failed to build entries: {}", e);
                return Err(e.into());
            }
        };

        self.sns_client
            .publish_batch()
            .topic_arn(&self.topic_arn)
            .set_publish_batch_request_entries(Some(entries))
            .send()
            .await
            .tap_err(|e| {
                log::error!(
                    "Failed to publish events to topic: {} : {e}",
                    self.topic_arn
                )
            })
            .tap_ok(log_publish_result)
            .map_err(SnsPublisherError::from)
            .map(|_| Ok(()))?
    }
}

fn log_publish_result(out: &PublishBatchOutput) {
    out.failed().iter().for_each(|entry| {
        log::warn!(
            "Failed to publish event: {}. Error: {:?}",
            entry.id(),
            SqsBatchError::from(entry)
        );
    });
    out.successful().iter().for_each(|entry| {
        log::info!("Published event: {:?}", entry.id());
    });
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
