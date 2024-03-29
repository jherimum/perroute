use crate::routes::routes;
use actix_web::{dev::Server, web::Data, App, HttpServer};
use anyhow::Result;
use derive_getters::Getters;
use perroute_commons::configuration::settings::{ServerSettings, Settings};
use perroute_connectors::Plugins;
use perroute_cqrs::{command_bus::bus::CommandBus, query_bus::bus::QueryBus};
use perroute_storage::connection_manager::ConnectionManager;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

#[derive(Clone, Getters, Debug)]
pub struct AppState {
    command_bus: CommandBus,
    query_bus: QueryBus,
    plugins: Plugins,
}

impl AppState {
    pub async fn from_settings(settings: &Settings) -> Result<Self> {
        let pool = ConnectionManager::build_pool(&settings.database).await?;
        Ok(Self {
            plugins: Plugins::full(),
            command_bus: CommandBus::complete(pool.clone(), Plugins::full()),
            query_bus: QueryBus::complete(pool),
        })
    }
}

impl From<PgPool> for AppState {
    fn from(value: PgPool) -> Self {
        Self {
            plugins: Plugins::full(),
            command_bus: CommandBus::complete(value.clone(), Plugins::full()),
            query_bus: QueryBus::complete(value),
        }
    }
}
pub struct Application {
    server: Server,
}

impl Application {
    fn listener(settings: &ServerSettings) -> Result<TcpListener, std::io::Error> {
        TcpListener::bind(format!("{}:{}", settings.host, settings.port))
    }

    pub async fn build(settings: &Settings) -> Result<Self, std::io::Error> {
        let pool = ConnectionManager::build_pool(&settings.database)
            .await
            .unwrap();
        let listener = Self::listener(&settings.server).unwrap();

        Ok(Self {
            server: server(listener, pool)?,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn server(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(routes().app_data(Data::<AppState>::new(pool.clone().into())))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
