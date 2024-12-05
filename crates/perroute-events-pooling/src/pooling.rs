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
    async fn fetch_events(&self) -> Result<Vec<DbEvent>, PoolingError> {
        Ok(
            EventRepository::unconsumed(&self.repository, self.max_events as usize)
                .await
                .tap_err(|e| {
                    log::error!("Failed to retrieve uncosumed messages from database: {e}")
                })?,
        )
    }

    async fn set_consumed(&self, events: Vec<DbEvent>) -> Result<(), PoolingError> {
        if events.is_empty() {
            return Ok(());
        };

        Ok(
            EventRepository::set_consumed(
                &self.repository,
                &Entity::ids(&events),
                Timestamp::now(),
            )
            .await
            .tap_err(|e| log::error!("Failed to set messages as consumed: {e}"))?,
        )
    }

    async fn inner_run(&self) -> Result<(), PoolingError> {
        log::info!("Starting to pooling events");

        let events = match self.fetch_events().await {
            Ok(events) if events.is_empty() => {
                log::info!("There are no events to be pooled");
                return Ok(());
            }
            Ok(events) => {
                log::info!("{} events were pooled from database", events.len());
                if log::log_enabled!(log::Level::Debug) {
                    log::debug!("Pooled events: {events:?}");
                }
                events
            }
            Err(e) => {
                log::error!("Error while pooling events from database: {e}");
                return Err(e.into());
            }
        };

        log::info!("Starting to publish events...");
        match events
            .iter()
            .filter(|e| self.publisheable_event_types.contains(e.event_type()))
            .map(Event::try_from)
            .collect::<Result<Vec<Event>, _>>()
        {
            Ok(publishable_events) if publishable_events.is_empty() => {
                log::info!("There is no events to publish. Check which events are publishable on the configuration");
            }
            Ok(publishable_events) => {
                log::info!(
                    "There are {} events to be published",
                    publishable_events.len()
                );

                if log::log_enabled!(log::Level::Debug) {
                    log::debug!("Events to be published: {publishable_events:?}");
                }

                self.publisher
                    .publish(publishable_events)
                    .await
                    .tap_err(|e| log::error!("Failed to publish events: {e}"))?;
            }
            Err(e) => {
                log::error!("Error while converting events to publishable events: {e}");
                return Err(e.into());
            }
        }

        self.set_consumed(events)
            .await
            .tap_err(|e| log::error!("Failed to set events consumed: {e}"))?;

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
