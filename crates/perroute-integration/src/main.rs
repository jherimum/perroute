use std::{error::Error, sync::Arc};

use perroute_commandbus::bus::CommandBus;
use perroute_commons::{
    configuration::settings::Settings, tracing::init_tracing,
    types::template::handlebars::Handlebars,
};
use perroute_connectors::Plugins;
use perroute_messaging::{
    events::Event,
    rabbitmq::{
        connection::RabbitmqConnection,
        consumer::{Consumer, EventHandler},
    },
};
use perroute_storage::connection_manager::ConnectionManager;
use tap::TapFallible;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    init_tracing();
    let settings =
        Settings::load().tap_err(|e| tracing::error!("Error loading settings. Error: {e}"))?;

    let rab = RabbitmqConnection::connect_from_settings(&settings).await?;
    let pool = ConnectionManager::build_pool(&settings.database)
        .await
        .tap_err(|e| tracing::error!("Failed to build connection poll:{e}"))?;
    let plugins = Plugins::full();
    let template_render = Arc::new(Handlebars::default());
    let command_bus = CommandBus::new(pool, plugins, template_render);
    let event_handler = DistributeMessageEventHandler { command_bus };

    let consumer = Consumer {
        connection: &rab,
        queue: "distribute-message",
        exchange: "perroute.events",
        routing_key: "MessageCreated",
        tag: "consumer",
        handler: &event_handler,
    };
    consumer.start().await;

    Ok(())
}

pub struct DistributeMessageEventHandler {
    command_bus: CommandBus,
}

#[async_trait::async_trait]
impl EventHandler for DistributeMessageEventHandler {
    async fn handle(&self, event: Event) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("{event:?}");
        Ok(())
    }
}
