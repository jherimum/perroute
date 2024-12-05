use crate::publisher::{Publisher, PublisherError};
use perroute_commons::{
    events::{Event, EventType},
    types::{entity::Entity, Timestamp},
};
use perroute_storage::{
    models::event::DbEvent,
    repository::{events::EventRepository, Repository},
};
use std::collections::HashSet;
use std::time::Duration;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum PoolingError {
    #[error("{0}")]
    RepositoryError(#[from] perroute_storage::repository::Error),

    #[error("{0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("{0}")]
    PublisherError(#[from] PublisherError),
}

pub struct Pooling<R, P> {
    repository: R,
    publisher: P,
    interval: Duration,
    max_events: u64,
    publisheable_event_types: HashSet<EventType>,
}

impl<R: Repository + Send + Sync, P: Publisher + Send + Sync> Pooling<R, P> {
    pub fn new(
        repository: R,
        publisher: P,
        interval: u64,
        max_events: u64,
        publisheable_event_types: HashSet<EventType>,
    ) -> Self {
        Self {
            repository,
            publisher,
            interval: Duration::from_secs(interval),
            max_events,
            publisheable_event_types,
        }
    }
    async fn fetch_events(&self) -> Result<Vec<Event>, PoolingError> {
        Ok(
            EventRepository::unconsumed(&self.repository, self.max_events as usize)
                .await
                .tap_err(|e| {
                    log::error!("Failed to retrieve uncosumed messages from database: {e}")
                })?
                .into_iter()
                .map(|e| Event::try_from(&e))
                .collect::<Result<Vec<_>, _>>()?,
        )
    }

    // async fn set_consumed(&self, events: Vec<DbEvent>) -> Result<(), PoolingError> {
    //     if events.is_empty() {
    //         return Ok(());
    //     };

    //     Ok(
    //         EventRepository::set_consumed(
    //             &self.repository,
    //             &Entity::ids(&events),
    //             Timestamp::now(),
    //         )
    //         .await
    //         .tap_err(|e| log::error!("Failed to set messages as consumed: {e}"))?,
    //     )
    // }

    async fn set_consumed1(
        &self,
        published: &Vec<Event>,
        skipped: &Vec<Event>,
    ) -> Result<(), PoolingError> {
        match (published, skipped) {
            (published, skipped) if published.is_empty() && skipped.is_empty() => Ok(()),
            (published, skipped) => {
                let tx = self.repository.begin().await?;

                if !published.is_empty() {
                    EventRepository::set_consumed(&tx, events, skipped, timestamp)
                }

                if !skipped.is_empty() {
                    EventRepository::set_consumed(&tx, events, skipped, timestamp)
                }

                self.set_consumed(events).await
            }
        }
    }

    async fn inner_run(&self) -> Result<(), PoolingError> {
        log::info!("Starting to pooling events");

        let (to_publish, ignored): (Vec<_>, Vec<_>) = self
            .fetch_events()
            .await?
            .into_iter()
            .partition(|e| self.publisheable_event_types.contains(e.event_type()));

        let output = self.publisher.publish(to_publish).await?;
        let published = output.success().to_owned();
        let not_published = output
            .failed()
            .into_iter()
            .map(|e| e.0.to_owned())
            .chain(ignored)
            .collect::<Vec<_>>();

        self.set_consumed1(&published, &not_published).await?;

        Ok(())
    }

    pub async fn run(self) {
        loop {
            if let Err(e) = self.inner_run().await {
                log::error!("Error while running pooling: {e}");
            }
            tokio::time::sleep(self.interval).await;
        }
    }
}
