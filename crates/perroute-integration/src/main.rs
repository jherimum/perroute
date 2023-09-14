use perroute_commons::{configuration::settings::Settings, tracing::init_tracing};
use perroute_messaging::rabbitmq::connection::RabbitmqConnection;
use perroute_storage::{connection_manager::ConnectionManager, error::StorageError};
use sqlx::PgPool;
use tap::TapFallible;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    init_tracing();
    let settings =
        Settings::load().tap_err(|e| tracing::error!("Error loading settings. Error: {e}"))?;

    let publisher = conn(&settings).await?;
    let pool = build_pool(&settings).await?;

    Ok(())
}

async fn conn(settings: &Settings) -> Result<RabbitmqConnection, anyhow::Error> {
    Ok(RabbitmqConnection::connect(settings.into()).await?)
}

async fn build_pool(settings: &Settings) -> Result<PgPool, StorageError> {
    ConnectionManager::build_pool(&settings.database)
        .await
        .tap_err(|e| tracing::error!("Failed to build connection poll:{e}"))
}
