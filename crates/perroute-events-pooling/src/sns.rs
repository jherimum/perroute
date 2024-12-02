use crate::publisher::{Publisher, PublisherResult};
use aws_sdk_sns::{
    config::http::HttpResponse,
    error::{BuildError, SdkError},
    operation::publish_batch::{PublishBatchError, PublishBatchOutput},
    types::{BatchResultErrorEntry, MessageAttributeValue, PublishBatchRequestEntry},
};
use perroute_commons::events::{Event, EventData};
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

fn to_entry(db_event: &Event) -> Result<PublishBatchRequestEntry, SnsPublisherError> {
    let event_data: &EventData = db_event.as_ref();
    Ok(PublishBatchRequestEntry::builder()
        .id(event_data.id())
        .message_group_id(event_data.entity_id())
        .message_deduplication_id(event_data.id())
        .message(serde_json::to_string(&db_event)?)
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
