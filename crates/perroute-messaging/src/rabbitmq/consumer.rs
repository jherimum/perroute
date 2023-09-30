use crate::events::Event;

use super::connection::{RabbitmqChannel, RabbitmqConnection};
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
};
use std::error::Error;

#[async_trait::async_trait]
pub trait EventHandler {
    async fn handle(&self, event: Event) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct Consumer<'c, 's, 'e> {
    pub connection: &'c RabbitmqConnection,
    pub queue: &'s str,
    pub exchange: &'s str,
    pub routing_key: &'s str,
    pub tag: &'s str,
    pub handler: &'e dyn EventHandler,
}

impl<'c, 's, 'e> Consumer<'c, 's, 'e> {
    pub async fn start(&self) {
        let channel = self.connection.create_channel();
        channel
            .get()
            .await
            .unwrap()
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
            .get()
            .await
            .unwrap()
            .queue_bind(
                self.queue,
                self.exchange,
                self.routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        let r = consume(
            &channel,
            &self.queue.to_owned(),
            &self.tag.to_owned(),
            self.handler,
        )
        .await;
    }
}

async fn consume<'e>(
    channel: &RabbitmqChannel,
    queue: &str,
    consumer_tag: &str,
    handler: &'e dyn EventHandler,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut consumer = channel
        .get()
        .await
        .unwrap()
        .basic_consume(
            &queue,
            &consumer_tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(recipient) = consumer.next().await {
        match recipient {
            Ok(r) => {
                tracing::info!("Mensagem recebida");
                let event = serde_json::from_slice::<Event>(&r.data).unwrap();
                handler.handle(event).await.unwrap();
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
