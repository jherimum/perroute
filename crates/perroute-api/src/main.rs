use perroute_api::{app::Application, rest::RestService};
use perroute_command_bus::create_command_bus;
use perroute_commons::configuration::settings::Settings;
use perroute_query_bus::create_query_bus;
use perroute_storage::create_repository;
use std::{error::Error, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    env_logger::init();

    let settings = Settings::load()?;

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let repository = create_repository(&settings.database).await?;
    let command_bus = create_command_bus(repository.clone());
    let query_bus = create_query_bus(repository);
    let rest_service = RestService::new(command_bus, query_bus);

    let app = Application::new(listener, rest_service)?;

    tokio::spawn(app.start()).await??;

    Ok(())
}
