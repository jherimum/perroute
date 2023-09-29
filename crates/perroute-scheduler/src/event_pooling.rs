use chrono::Utc;
use perroute_messaging::events::{EventPublisher, EventPublisherError};
use perroute_storage::{error::StorageError, models::db_event::DbEvent};
use sqlx::PgPool;
use std::{fmt::Debug, sync::Arc};
use tap::TapFallible;
use tokio::sync::Semaphore;
use tokio_cron_scheduler::{Job, JobSchedulerError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error(transparent)]
    Producer(#[from] EventPublisherError),
}

#[derive(Debug, Clone)]
pub struct EventPooling<P> {
    pool: PgPool,
    batch_size: i64,
    cron: String,
    publisher: P,
    semaphore: Arc<Semaphore>,
}

impl<P: EventPublisher> EventPooling<P> {
    pub async fn new(pool: PgPool, batch_size: i64, cron: String, publisher: P) -> Self {
        Self {
            pool,
            batch_size,
            cron,
            publisher,
            semaphore: Arc::new(Semaphore::new(1)),
        }
    }
}

impl<P: EventPublisher> EventPooling<P> {
    async fn publish_event(&self, event: &DbEvent) -> Result<(), EventPublisherError> {
        self.publisher.publish(&event.into()).await
    }

    pub async fn run(&self) -> Result<(), Error> {
        let acquire = self.semaphore.try_acquire();
        if acquire.is_err() {
            tracing::warn!("Event polling already running");
            return Ok(());
        }

        let pending_events = DbEvent::fetch_unconsumed(&self.pool, self.batch_size)
            .await
            .tap_err(|e| tracing::error!("Failed to fetch events: {e}"))?;

        if pending_events.is_empty() {
            tracing::info!("No pending events");
            return Ok(());
        }
        for event in pending_events {
            tracing::info!("Consuming event: {}", event.id());
            let mut tx = self
                .pool
                .begin()
                .await
                .tap_err(|e| {
                    tracing::error!("Failed to start transaction: {e}");
                })
                .map_err(StorageError::Tx)?;

            let event = event
                .set_consumed_at(Utc::now().naive_utc())
                .update(tx.as_mut())
                .await
                .tap_err(|e| {
                    tracing::error!("Failed to update event: {e}");
                })?;

            match self.publish_event(&event).await {
                Ok(_) => {
                    tracing::info!("Event {} published. Commiting...", event.id());
                    tx.commit()
                        .await
                        .tap_err(|e| {
                            tracing::error!("Failed to commit transaction: {e}");
                        })
                        .map_err(StorageError::Tx)?;
                }
                Err(e) => {
                    tracing::error!("Failed to publish event: {e}. Rolling back...");
                    tx.rollback()
                        .await
                        .tap_err(|e| {
                            tracing::error!("Failed to rollback transaction: {e}");
                        })
                        .map_err(StorageError::Tx)?;
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }
}

impl<P: Clone + EventPublisher + Send + Sync + 'static> TryInto<Job> for EventPooling<P> {
    type Error = JobSchedulerError;

    fn try_into(self) -> Result<Job, Self::Error> {
        let x = Job::new_async(self.cron.clone().as_str(), move |_uuid, mut _l| {
            let pooling = self.clone();
            Box::pin(async move {
                if let Err(e) = pooling.run().await {
                    tracing::error!("Error polling events: {e}");
                }
            })
        })?;
        Ok(x)
    }
}
