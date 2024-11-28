use crate::publisher::{Publisher, PublisherResult};
use aws_sdk_sns::{
    config::http::HttpResponse,
    error::{BuildError, SdkError},
    operation::publish_batch::PublishBatchError,
    types::{BatchResultErrorEntry, PublishBatchRequestEntry},
};
use perroute_commons::events::Event;
use perroute_storage::models::event::DbEvent;

#[derive(Debug, thiserror::Error)]
pub enum SnsPublisherError {
    #[error("Failed to publish events: {0:?}")]
    PublishError(#[from] SdkError<PublishBatchError, HttpResponse>),

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
    async fn publish<'e>(&self, events: &'e Vec<DbEvent>) -> PublisherResult {
        let entries = events
            .iter()
            .filter_map(|event| match to_entry(event) {
                Ok(entry) => Some(entry),
                Err(error) => {
                    log::error!("Failed to build entry: {}", error);
                    None
                }
            })
            .collect::<Vec<_>>();

        if entries.is_empty() {
            return Ok(());
        }

        let sqs_output = self
            .sns_client
            .publish_batch()
            .topic_arn(&self.topic_arn)
            .set_publish_batch_request_entries(Some(entries))
            .send()
            .await
            .map_err(SnsPublisherError::from)?;

        if let Some(failed) = sqs_output.failed {
            for error in failed {
                log::error!(
                    "Failed to publish event {}. Error: {}",
                    error.id(),
                    SqsBatchError::from(&error)
                );
            }
        }

        Ok(())
    }
}

fn to_entry(db_event: &DbEvent) -> Result<PublishBatchRequestEntry, SnsPublisherError> {
    Ok(PublishBatchRequestEntry::builder()
        .id(db_event.id())
        .message_group_id(db_event.entity_id())
        .message_deduplication_id(db_event.entity_id())
        .message(serde_json::to_string(&Event::from(db_event))?)
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
