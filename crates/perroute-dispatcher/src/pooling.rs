use crate::dispatcher::Dispatcher;
use aws_sdk_sqs::{
    config::http::HttpResponse,
    error::SdkError,
    operation::receive_message::{ReceiveMessageError, ReceiveMessageOutput},
    types::Message,
};
use perroute_commons::events::{ApplicationEventData, MessageCreatedEvent};
use perroute_storage::repository::Repository;
use perroute_template::{render::TemplateRenderPlugin, repository::TemplateLookup};
use std::{sync::Arc, time::Duration};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum PooligError {
    #[error("Error receiving message: {0}")]
    ReceiveMessageError(#[from] SdkError<ReceiveMessageError, HttpResponse>),

    #[error("Error running task: {0}")]
    TaskError(#[from] tokio::task::JoinError),
}

pub struct SqsPooling<REPO, TRP, TR> {
    interval: Duration,
    pool_size: i32,
    sqs_client: Arc<aws_sdk_sqs::Client>,
    queue_url: String,
    dispatcher: Dispatcher<REPO, TRP, TR>,
}

impl<REPO, TRP, TR> SqsPooling<REPO, TRP, TR>
where
    TRP: TemplateRenderPlugin + Clone + Send + Sync + 'static,
    TR: TemplateLookup + Clone + Send + Sync + 'static,
    REPO: Repository + Clone + Send + Sync + 'static,
{
    pub fn new(
        dispatcher: Dispatcher<REPO, TRP, TR>,
        sqs_client: aws_sdk_sqs::Client,
        queue_url: &str,
        interval: Duration,
        pool_size: i32,
    ) -> Self {
        SqsPooling {
            dispatcher,
            sqs_client: Arc::new(sqs_client),
            interval,
            pool_size,
            queue_url: queue_url.to_string(),
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

    async fn receive_message(
        &self,
    ) -> Result<ReceiveMessageOutput, PooligError> {
        self.sqs_client
            .receive_message()
            .queue_url(&self.queue_url)
            .max_number_of_messages(self.pool_size)
            .send()
            .await
            .tap_err(|e| log::error!("Failed to receive messages from sqs:{e}"))
            .map_err(Into::into)
    }

    async fn inner_run(&self) -> Result<(), PooligError> {
        let tasks = self
            .receive_message()
            .await?
            .messages
            .unwrap_or_default()
            .into_iter()
            .filter_map(|message| {
                to_event(&message).map(|event| (message, event))
            })
            .map(|(message, event)| {
                let dispatcher = self.dispatcher.clone();
                (message, tokio::spawn(dispatcher.dispatch(event)))
            })
            .collect::<Vec<_>>();

        for (message, task) in tasks {
            match task.await {
                Ok(Ok(_)) => {
                    self.delete_message(&message).await;
                }
                Ok(Err(e)) => {
                    //todo: send to a dlq?
                    log::error!("Task failed: {e}");
                }
                Err(e) => {
                    //todo: send to a dlq?
                    log::error!("Task completation failed: {e}");
                }
            }
        }

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

fn to_event(
    message: &Message,
) -> Option<ApplicationEventData<MessageCreatedEvent>> {
    message.body().and_then(|body| {
        serde_json::from_str(body)
            .tap_err(|e| log::error!("Failed to parse message body: {e}"))
            .ok()
    })
}
