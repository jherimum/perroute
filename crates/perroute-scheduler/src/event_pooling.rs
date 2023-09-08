use chrono::Utc;
use perroute_storage::{error::StorageError, models::db_event::DbEvent};
use sqlx::PgPool;
use tokio_cron_scheduler::{Job, JobSchedulerError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Storage(#[from] StorageError),
}

#[derive(Debug, Clone)]
pub struct EventPooling {
    pool: PgPool,
    batch_size: i64,
    cron: String,
}

impl EventPooling {
    pub fn new(pool: PgPool, batch_size: i64, cron: String) -> Self {
        Self {
            pool,
            batch_size,
            cron,
        }
    }
}

impl EventPooling {
    pub async fn run(&self) -> Result<(), Error> {
        let pending_events = DbEvent::fetch_unconsumed(&self.pool, self.batch_size)
            .await
            .unwrap();
        if pending_events.is_empty() {
            tracing::info!("No pending events");
            return Ok(());
        }
        for event in pending_events {
            tracing::info!("Consuming event: {event:?}");
            let mut tx = self.pool.begin().await.unwrap();

            let event = event
                .set_consumed_at(Utc::now().naive_utc())
                .update(&mut tx)
                .await
                .unwrap();

            tx.commit().await.unwrap();
            tracing::info!("Event: {event:?} consumed");
        }
        Ok(())
    }
}

impl TryInto<Job> for EventPooling {
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
