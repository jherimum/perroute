use aws_sdk_sqs::types::{DeleteMessageBatchRequestEntry, Message};
use futures::stream::FuturesUnordered;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum PooligError {}

pub struct SqsPooling {
    sqs_client: aws_sdk_sqs::Client,
    s3_client: aws_sdk_s3::Client,
    interval: Duration,
    pool_size: i32,
    queue_url: String,
}

impl SqsPooling {
    pub fn new(
        //repository: R,
        sqs_client: aws_sdk_sqs::Client,
        s3_client: aws_sdk_s3::Client,
        queue_url: &str,
        interval: Duration,
        pool_size: i32,
    ) -> Self {
        SqsPooling {
            //repository,
            sqs_client,
            s3_client,
            interval,
            pool_size,
            queue_url: queue_url.to_string(),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let messages_output = self
                .sqs_client
                .receive_message()
                .queue_url(&self.queue_url)
                .max_number_of_messages(self.pool_size)
                .send()
                .await;

            println!("pool");

            match messages_output {
                Ok(messages) => {
                    let messages = messages
                        .messages
                        .unwrap_or_default()
                        .into_iter()
                        .map(|m| tokio::spawn(task(m)))
                        .collect::<FuturesUnordered<_>>();
                    let tasks_result = futures::future::join_all(messages).await;

                    if tasks_result.is_empty() {
                        continue;
                    }
                    let mut delete = self
                        .sqs_client
                        .delete_message_batch()
                        .queue_url(&self.queue_url);

                    for r in tasks_result {
                        let entry = DeleteMessageBatchRequestEntry::builder()
                            .id("input")
                            .receipt_handle(r.unwrap().receipt_handle().unwrap_or_default())
                            .build()
                            .unwrap();
                        delete = delete.entries(entry);
                    }

                    println!("{:?}", delete.send().await);
                }

                Err(e) => {
                    eprint!("Error: {:?}", e);
                }
            }
            tokio::time::sleep(self.interval).await;
        }
    }
}

async fn task(message: Message) -> Message {
    //println!("{:?}", message.body());
    message
}
