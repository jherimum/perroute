use perroute_commons::types::{entity::Entity, id::Id, Timestamp};
use perroute_storage::{
    models::event::DbEvent,
    repository::{
        events::{EventQuery, EventRepository},
        Repository,
    },
};
use std::error::Error;
use std::time::Duration;

use crate::publisher::Publisher;

pub struct Pooling<R, P> {
    repository: R,
    publisher: P,
    interval: Duration,
    max_events: u64,
}

impl<R: Repository + Send + Sync, P: Publisher + Send + Sync> Pooling<R, P> {
    pub fn new(repository: R, publisher: P, interval: u64, max_events: u64) -> Self {
        Self {
            repository,
            publisher,
            interval: Duration::from_secs(interval),
            max_events,
        }
    }
    async fn fetch_events(&self) -> Result<Vec<DbEvent>, perroute_storage::repository::Error> {
        EventRepository::query(
            &self.repository,
            &EventQuery::Unconsumed,
            self.max_events as usize,
        )
        .await
    }

    async fn set_consumed(&self, events: Vec<DbEvent>) -> Result<(), Box<dyn Error>> {
        if events.is_empty() {
            return Ok(());
        };

        Ok(
            EventRepository::set_consumed(
                &self.repository,
                &Entity::ids(&events),
                Timestamp::now(),
            )
            .await?,
        )
    }

    async fn inner_run(&self) -> Result<(), Box<dyn Error>> {
        let pooled_events = self.fetch_events().await?;
        if pooled_events.is_empty() {
            log::info!("No events to pool");
            return Ok({});
        }

        log::debug!(
            "Pooled events from database: {:?}",
            pooled_events.iter().map(|e| e.id()).collect::<Vec<_>>()
        );

        self.publisher.publish(&pooled_events).await?;

        self.set_consumed(pooled_events).await?;

        Ok(())
    }

    pub async fn run(self) {
        loop {
            log::info!("Pooling events");
            if let Err(e) = self.inner_run().await {
                log::error!("Error while running pooling: {}", e);
            }
            tokio::time::sleep(self.interval).await;
        }
    }
}
