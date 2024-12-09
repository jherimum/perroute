use perroute_commons::{configuration::settings::Settings, events::EventType};
use perroute_events_pooling::{pooling::Pooling, sns::SnsPublisher};
use perroute_storage::create_repository;
use std::error::Error;
use tap::TapFallible;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    env_logger::init();

    log::info!("Starting events pooling service...");
    let settings = Settings::load().tap_err(|e| log::error!("Failed to load settings: {e}"))?;
    let event_pooling_settings = settings
        .pooling
        .ok_or("Event pooling settings are missing")?;

    let sdk_config = aws_config::load_from_env().await;
    let sns_client = aws_sdk_sns::Client::new(&sdk_config);
    let repository = create_repository(&settings.database.unwrap())
        .await
        .tap_err(|e| log::error!("Failed to create repository: {e}"))?;

    let publisher = SnsPublisher::new(sns_client, event_pooling_settings.topic_arn);

    let pooling = Pooling::new(
        repository.clone(),
        publisher,
        event_pooling_settings.interval,
        event_pooling_settings.max_events,
        EventType::parse(
            &event_pooling_settings
                .publishable_events
                .unwrap_or("".to_string()),
        )
        .tap_err(|e| log::error!("Failed to parse publishable events: {e}"))?,
    );

    tokio::spawn(async move { pooling.run().await })
        .await
        .tap_err(|e| log::error!("Failed to join task: {e}"))?;

    Ok(())
}
