use crate::publisher::{Publisher, PublisherError};
use perroute_commons::{
    events::{Event, EventType},
    types::{entity::Entity, id::Id, Timestamp},
};
use perroute_storage::{
    active_record::{
        datasource::{DataSource, NonTransactionalDataSource},
        event::EventActiveRecord,
        ActiveRecordError,
    },
    models::event::DbEvent,
};
use std::collections::HashSet;
use std::time::Duration;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum PoolingError {
    #[error("{0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("{0}")]
    PublisherError(#[from] PublisherError),

    #[error("{0}")]
    ActiveRecordError(
        #[from] perroute_storage::active_record::ActiveRecordError,
    ),
}

pub struct Pooling<P> {
    datasource: DataSource<NonTransactionalDataSource>,
    publisher: P,
    interval: Duration,
    max_events: u64,
    publishable_event_types: HashSet<EventType>,
}

impl<P: Publisher + Send + Sync> Pooling<P> {
    pub fn new(
        datasource: DataSource<NonTransactionalDataSource>,
        publisher: P,
        interval: u64,
        max_events: u64,
        publishable_event_types: HashSet<EventType>,
    ) -> Self {
        Self {
            datasource,
            publisher,
            interval: Duration::from_secs(interval),
            max_events,
            publishable_event_types,
        }
    }

    async fn set_consumed(
        &self,
        published: Vec<Id>,
        skipped: Vec<Id>,
    ) -> Result<(), PoolingError> {
        match (published, skipped) {
            (published, skipped)
                if published.is_empty() && skipped.is_empty() =>
            {
                Ok(())
            }
            (published, skipped) => {
                let tx = self.datasource.begin_transaction().await.unwrap();

                let consumed_at = Timestamp::now();
                if !published.is_empty() {
                    if let Err(e) = DbEvent::set_consumed(
                        &tx,
                        published,
                        false,
                        &consumed_at,
                    )
                    .await
                    {
                        tx.rollback().await.tap_err(|e| {
                            log::error!("Failed to rollback transaction:{e}")
                        })?;
                        return Err(e.into());
                    }
                }

                if !skipped.is_empty() {
                    if let Err(e) =
                        DbEvent::set_consumed(&tx, skipped, true, &consumed_at)
                            .await
                    {
                        tx.rollback().await.tap_err(|e| {
                            log::error!("Failed to rollback transaction:{e}")
                        })?;
                        return Err(e.into());
                    }
                }

                tx.commit().await.tap_err(|e| {
                    log::error!("Failet co commit transaction: {e}")
                })?;

                Ok(())
            }
        }
    }

    async fn fetch_events(&self) -> Result<Vec<DbEvent>, ActiveRecordError> {
        DbEvent::unconsumed(&self.datasource, self.max_events as usize)
            .await
            .tap_err(|e| {
                log::error!(
                    "Failed to retrieve uncosumed messages from database: {e}"
                )
            })
    }

    async fn inner_run(&self) -> Result<(), PoolingError> {
        log::info!("Starting to pooling events from database");

        let (to_publish, ignored): (Vec<_>, Vec<_>) = self
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
                Event::try_from(&db_event)
                    .tap_err(|error| log::error!("Failed to convert dbevent {db_event:?} to event: {error}"))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .partition(|e| self.publishable_event_types.contains(e.event_type()));

        let output = self
            .publisher
            .publish(to_publish.iter().collect())
            .await
            .tap_err(|e| log::error!("Failed to publish events: {e}"))?;

        let published = output.success();
        let skipped = output
            .failed()
            .iter()
            .map(|e| e.0.to_owned())
            .chain(ignored)
            .collect::<Vec<_>>();

        self.set_consumed(
            published.iter().map(|e| e.id().to_owned()).collect(),
            skipped.into_iter().map(|e| e.id().to_owned()).collect(),
        )
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
