use crate::{digestor::Digesters, MessageDigest};
use aws_sdk_s3::{
    config::http::HttpResponse, operation::put_object::PutObjectError, primitives::ByteStream,
};
use aws_sdk_sqs::{
    error::SdkError,
    operation::receive_message::{ReceiveMessageError, ReceiveMessageOutput},
    types::Message,
};
use perroute_commons::{
    events::{ApplicationEventData, MessageCreatedEvent},
    template::TemplateRender,
};
use perroute_connectors::ProviderPluginRepository;
use perroute_storage::repository::Repository;
use std::{sync::Arc, time::Duration};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum PooligError {
    #[error("Error receiving message: {0}")]
    ReceiveMessageError(#[from] SdkError<ReceiveMessageError, HttpResponse>),

    #[error("Error running task: {0}")]
    TaskError(#[from] tokio::task::JoinError),
}

pub struct SqsPooling<R, TR, PR> {
    interval: Duration,
    pool_size: i32,
    sqs_client: Arc<aws_sdk_sqs::Client>,
    queue_url: String,
    s3_client: Arc<aws_sdk_s3::Client>,
    digesters: Digesters<R, TR, PR>,
    bucket_name: String,
}

impl<R, TR, PR> SqsPooling<R, TR, PR>
where
    R: Repository + Clone + Send + Sync + 'static,
    TR: TemplateRender + Send + Sync + 'static,
    PR: ProviderPluginRepository + Send + Sync + 'static,
{
    pub fn new(
        repository: R,
        template_render: TR,
        sqs_client: aws_sdk_sqs::Client,
        s3_client: aws_sdk_s3::Client,
        queue_url: &str,
        interval: Duration,
        pool_size: i32,
        bucket_name: &str,
        plugin_repository: PR,
    ) -> Self {
        SqsPooling {
            sqs_client: Arc::new(sqs_client),
            s3_client: Arc::new(s3_client),
            interval,
            pool_size,
            queue_url: queue_url.to_string(),
            digesters: Digesters::new(repository, template_render, plugin_repository),
            bucket_name: bucket_name.to_string(),
        }
    }

    async fn delete_message(&self, message: &Message) {
        if let Some(receipt_handle) = message.receipt_handle() {
            match self
                .sqs_client
                .delete_message()
                .queue_url(&self.queue_url)
                .receipt_handle(receipt_handle)
                .send()
                .await
            {
                Ok(_) => {
                    log::info!("Message deleted successfully");
                }
                Err(e) => {
                    log::error!("Failed to delete message: {e}");
                }
            }
        } else {
            log::warn!("Failed to delete message: receipt handle not found");
        }
    }

    async fn receive_message(&self) -> Result<ReceiveMessageOutput, PooligError> {
        self.sqs_client
            .receive_message()
            .queue_url(&self.queue_url)
            .max_number_of_messages(self.pool_size)
            .send()
            .await
            .tap_err(|e| log::error!("Failed to receive messages from sqs:{e}"))
            .map_err(Into::into)
    }

    async fn store_message_digest(
        &self,
        data: &String,
    ) -> Result<(), SdkError<PutObjectError, HttpResponse>> {
        self.s3_client
            .put_object()
            .bucket(&self.bucket_name)
            .body(ByteStream::from(data.as_bytes().to_vec()))
            .send()
            .await
            .map(|_| ())
    }

    async fn handle_digest_result(&self, message: &Message, result: MessageDigest) {
        let json = serde_json::to_string(&result).unwrap();
        self.store_message_digest(&json).await.unwrap();
        self.delete_message(message).await;
    }

    async fn inner_run(&self) -> Result<(), PooligError> {
        let tasks = self
            .receive_message()
            .await?
            .messages
            .unwrap_or_default()
            .into_iter()
            .filter_map(|message| to_event(&message).map(|e| (message, e)))
            .map(|(message, event)| (message, self.digesters.create(event)))
            .map(|(message, digestor)| (message, tokio::task::spawn(digestor.execute())))
            .collect::<Vec<_>>();

        // for (message, task) in tasks {
        //     match task.await {
        //         Ok(Ok(digest)) => {
        //             self.handle_digest_result(&message, digest).await;
        //         }
        //         Ok(Err(e)) => {
        //             log::error!("Task failed: {e}");
        //         }
        //         Err(e) => {
        //             log::error!("Task completation failed: {e}");
        //         }
        //     }
        // }

        Ok(())
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match self.inner_run().await {
                Ok(_) => {}
                Err(e) => {
                    log::error!("Error running pooling: {e}");
                }
            }

            tokio::time::sleep(self.interval).await;
        }
    }
}

fn to_event(message: &Message) -> Option<ApplicationEventData<MessageCreatedEvent>> {
    message.body().as_deref().and_then(|body| {
        serde_json::from_str(body)
            .tap_err(|e| log::error!("Failed to parse message body: {e}"))
            .ok()
    })
}
