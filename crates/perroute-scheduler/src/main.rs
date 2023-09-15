use perroute_commons::{configuration::settings::Settings, tracing::init_tracing};
use perroute_messaging::rabbitmq::{connection::RabbitmqConnection, RabbitmqEventPublisher};
use perroute_scheduler::event_pooling::EventPooling;
use perroute_storage::{connection_manager::ConnectionManager, error::StorageError};
use sqlx::PgPool;
use tap::TapFallible;
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    init_tracing();
    let settings =
        Settings::load().tap_err(|e| tracing::error!("Error loading settings. Error: {e}"))?;

    let publisher: RabbitmqEventPublisher = build_publisher(&settings).await?;
    let pool = build_pool(&settings).await?;

    let mut sched = JobScheduler::new()
        .await
        .tap_err(|e| tracing::error!("Failed to build scheduler: {e}"))?;

    let event_pooling_job: Job =
        EventPooling::new(pool.clone(), 2000, "1/3 * * * * *".to_string(), publisher)
            .await
            .try_into()
            .tap_err(|e| tracing::error!("Failed to build event pooling job: {e}"))?;

    sched.shutdown_on_ctrl_c();
    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    }));

    sched
        .add(event_pooling_job)
        .await
        .tap_err(|e| tracing::error!("Failed to add job: {e}"))?;

    sched.start().await.tap_err(|e| {
        tracing::error!("Failed to start scheduler: {e}");
    })?;

    loop {
        tracing::info!("I'm alive!");
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

async fn build_publisher(settings: &Settings) -> Result<RabbitmqEventPublisher, anyhow::Error> {
    let rabbitmq_conn = RabbitmqConnection::connect_from_settings(settings).await?;
    Ok(RabbitmqEventPublisher::new(rabbitmq_conn).await.unwrap())
}

async fn build_pool(settings: &Settings) -> Result<PgPool, StorageError> {
    ConnectionManager::build_pool(&settings.database)
        .await
        .tap_err(|e| tracing::error!("Failed to build connection poll:{e}"))
}
