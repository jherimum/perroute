use crate::{
    connection::{RecoverableChannel, RecoverableConnection},
    events::Event,
};
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
};
use std::error::Error;

pub struct Consumer<'c, 's, F>
where
    F: Fn(Event) + Send + Sync + 'static + Clone,
{
    pub connection: &'c RecoverableConnection,
    pub queue: &'s str,
    pub exchange: &'s str,
    pub routing_key: &'s str,
    pub tag: &'s str,
    pub threads: u32,
    pub function: F,
}

impl<'c, 's, F> Consumer<'c, 's, F>
where
    F: Fn(Event) + Send + Sync + 'static + Clone,
{
    pub async fn start(&self) {
        let channel = self.connection.create_recoverable_channel().await;
        channel
            .queue_declare(
                self.queue,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .unwrap();

        channel
            .queue_bind(
                self.queue,
                self.exchange,
                self.routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        for i in 1..self.threads {
            let channel = self.connection.create_recoverable_channel().await;
            let queue = self.queue.to_owned();
            let consumer_tag = self.tag.to_owned();
            let f = self.function.clone();
            tokio::spawn(async move {
                let mut res =
                    consume(&channel, queue.clone(), consumer_tag.clone(), f.clone(), i).await;
                while res.is_err() {
                    res =
                        consume(&channel, queue.clone(), consumer_tag.clone(), f.clone(), i).await;
                }
                tracing::error!(
                    "xxxxxxxxxxxx -------- Falha ao consumir mensagem - xxxxxxxxxxxxxxxxxx"
                )
            });
        }
    }
}

async fn consume<F: Fn(Event) + Send + Sync + 'static + Clone>(
    channel: &RecoverableChannel,
    queue: String,
    consumer_tag: String,
    f: F,
    i: u32,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut consumer = channel
        .get()
        .await
        .basic_consume(
            &queue,
            &format!("{}-{}", &consumer_tag, i),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(r) => {
                let event = serde_json::from_slice::<Event>(&r.data).unwrap();
                (f)(event.clone());
                r.ack(BasicAckOptions::default()).await.unwrap();
            }
            Err(e) => {
                tracing::error!("Falha ao consumir mensagem: {e}");
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}
