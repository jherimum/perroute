use crate::publisher::{Publisher, PublisherError};
use perroute_commons::{
    events::{Event, EventType},
    types::{entity::Entity, Timestamp},
};
use perroute_storage::{
    models::event::DbEvent,
    repository::{events::EventRepository, Repository, TransactedRepository},
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
    publishable_event_types: HashSet<EventType>,
}

impl<R: Repository + Send + Sync, P: Publisher + Send + Sync> Pooling<R, P> {
    pub fn new(
        repository: R,
        publisher: P,
        interval: u64,
        max_events: u64,
        publishable_event_types: HashSet<EventType>,
    ) -> Self {
        Self {
            repository,
            publisher,
            interval: Duration::from_secs(interval),
            max_events,
            publishable_event_types,
        }
    }

    async fn set_consumed(
        &self,
        published: &Vec<Event>,
        skipped: &Vec<Event>,
    ) -> Result<(), PoolingError> {
        match (published, skipped) {
            (published, skipped) if published.is_empty() && skipped.is_empty() => Ok(()),
            (published, skipped) => {
                let tx = self.repository.begin().await?;
                let consumed_at = Timestamp::now();
                if !published.is_empty() {
                    if let Err(e) = EventRepository::set_consumed(
                        &tx,
                        Entity::ids(&published),
                        false,
                        &consumed_at,
                    )
                    .await
                    {
                        tx.rollback().await?;
                        return Err(e.into());
                    }
                }

                if !skipped.is_empty() {
                    if let Err(e) = EventRepository::set_consumed(
                        &tx,
                        Entity::ids(&skipped),
                        true,
                        &consumed_at,
                    )
                    .await
                    {
                        tx.rollback().await?;
                        return Err(e.into());
                    }
                }

                tx.commit().await?;

                Ok(())
            }
        }
    }

    async fn fetch_events(&self) -> Result<Vec<DbEvent>, perroute_storage::repository::Error> {
        EventRepository::unconsumed(&self.repository, self.max_events as usize)
            .await
            .tap_err(|e| log::error!("Failed to retrieve uncosumed messages from database: {e}"))
    }

    async fn inner_run(&self) -> Result<(), PoolingError> {
        log::info!("Starting to pooling events from database");

        let (to_publish, ignored) = self
            .fetch_events()
            .await
            .tap_ok(|events| log::info!("{} events were pooled form database", events.len()))
            .tap_ok(|events| {
                if log::log_enabled!(log::Level::Debug) {
                    log::debug!("Events pooled from database: {:#?}", Entity::ids(events));
                }
            })
            .tap_err(|e| log::error!("Failed to fetch events from database: {e}"))?
            .into_iter()
            .map(|db_event| {
                Event::try_from(&db_event).tap_err(|error| {
                    log::error!("Failed to convert dbevent {db_event:?} to event: {error}")
                })
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .partition(|e| self.publishable_event_types.contains(&e.event_type()));

        let output = self
            .publisher
            .publish(to_publish)
            .await
            .tap_err(|e| log::error!("Failed to publish events: {e}"))?;

        let published = output.success();
        let skipped = output
            .failed()
            .into_iter()
            .map(|e| e.0.to_owned())
            .chain(ignored)
            .collect::<Vec<_>>();

        self.set_consumed(&published, &skipped)
            .await
            .tap_err(|e| log::error!("Failed to mark events as consumed: {e}"))?;

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
