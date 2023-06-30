use anyhow::Result;
use derive_getters::Getters;
use perroute_commons::configuration::settings::Settings;
use perroute_cqrs::{command_bus::bus::CommandBus, query_bus::bus::QueryBus};
use perroute_storage::connection_manager::ConnectionManager;

#[derive(Clone, Getters, Debug)]
pub struct AppState {
    command_bus: CommandBus,
    query_bus: QueryBus,
}

impl AppState {
    pub async fn from_settings(settings: &Settings) -> Result<Self> {
        let pool = ConnectionManager::build_pool(&settings.database).await?;
        Ok(AppState {
            command_bus: CommandBus::complete(pool.clone()),
            query_bus: QueryBus::complete(pool),
        })
    }
}
