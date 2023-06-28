use actix_web::{
    web::{self, Data},
    App, HttpServer, Scope,
};
use anyhow::{Context, Result};
use perroute_backoffice_actix::{
    routes::{
        channel::ChannelRouter, health::HealthRouter, message_type::MessageTypeRouter,
        schema::SchemaRouter,
    },
    AppState,
};
use perroute_commons::{configuration::settings::Settings, tracing::init_tracing};
use tap::TapFallible;
use tracing_actix_web::TracingLogger;

fn routes(state: AppState) -> Scope {
    web::scope("/api")
        .service(ChannelRouter::routes())
        .service(MessageTypeRouter::routes())
        .service(SchemaRouter::routes())
        .service(HealthRouter::routes())
        .app_data(Data::new(state))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    init_tracing();
    let settings =
        Settings::load().tap_err(|e| tracing::error!("Error loading settings. Error: {e}"))?;

    let state = AppState::from_settings(&settings).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(routes(state.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .with_context(|| "")?;

    Ok(())
}
