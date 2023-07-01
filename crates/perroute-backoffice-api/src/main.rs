use actix_web::{
    web::{self, Data},
    App, HttpServer, Scope,
};
use anyhow::{Context, Result};
use perroute_backoffice_api::{
    app::AppState,
    routes::{
        channel::{ChannelRouter, CHANNELS_RESOURCE_NAME, CHANNEL_RESOURCE_NAME},
        message_type::{
            MessageTypeRouter, MESSAGE_TYPES_RESOURCE_NAME, MESSAGE_TYPE_RESOURCE_NAME,
        },
        route::{RouteRouter, ROUTES_RESOURCE_NAME, ROUTE_RESOURCE_NAME},
        schema::{SchemaRouter, SCHEMAS_RESOURCE_NAME, SCHEMA_RESOURCE_NAME},
    },
};
use perroute_commons::{configuration::settings::Settings, tracing::init_tracing};
use tap::TapFallible;
use tracing_actix_web::TracingLogger;

fn routes(state: AppState) -> Scope {
    web::scope("/api")
        .service(
            web::scope("/v1").service(
                web::scope("/channels")
                    .service(
                        web::resource("")
                            .name(CHANNELS_RESOURCE_NAME)
                            .route(web::get().to(ChannelRouter::query))
                            .route(web::post().to(ChannelRouter::create_channel)),
                    )
                    .service(
                        web::scope("/{channel_id}")
                            .service(
                                web::resource("")
                                    .name(CHANNEL_RESOURCE_NAME)
                                    .route(web::get().to(ChannelRouter::find_channel))
                                    .route(web::put().to(ChannelRouter::update))
                                    .route(web::delete().to(ChannelRouter::delete)),
                            )
                            .service(
                                web::scope("/message_types")
                                    .service(
                                        web::resource("")
                                            .name(MESSAGE_TYPES_RESOURCE_NAME)
                                            .route(web::get().to(MessageTypeRouter::query))
                                            .route(web::post().to(MessageTypeRouter::create)),
                                    )
                                    .service(
                                        web::scope("/{message_type_id}").service(
                                            web::resource("")
                                                .name(MESSAGE_TYPE_RESOURCE_NAME)
                                                .route(web::get().to(MessageTypeRouter::find))
                                                .route(web::put().to(MessageTypeRouter::update))
                                                .route(web::delete().to(MessageTypeRouter::delete)),
                                        ),
                                    ),
                            )
                            .service(
                                web::scope("/schemas")
                                    .service(
                                        web::resource("")
                                            .name(SCHEMAS_RESOURCE_NAME)
                                            .route(web::get().to(SchemaRouter::query))
                                            .route(web::post().to(SchemaRouter::create)),
                                    )
                                    .service(
                                        web::scope("/{schema_id}").service(
                                            web::resource("")
                                                .name(SCHEMA_RESOURCE_NAME)
                                                .route(web::get().to(SchemaRouter::find))
                                                .route(web::put().to(SchemaRouter::update))
                                                .route(web::delete().to(SchemaRouter::delete)),
                                        ),
                                    ),
                            )
                            .service(
                                web::scope("/routes")
                                    .service(
                                        web::resource("")
                                            .name(ROUTES_RESOURCE_NAME)
                                            .route(web::get().to(RouteRouter::query))
                                            .route(web::post().to(RouteRouter::create)),
                                    )
                                    .service(
                                        web::scope("/{route_id}").service(
                                            web::resource("")
                                                .name(ROUTE_RESOURCE_NAME)
                                                .route(web::get().to(RouteRouter::find))
                                                .route(web::put().to(RouteRouter::update))
                                                .route(web::delete().to(RouteRouter::delete)),
                                        ),
                                    ),
                            ),
                    ),
            ),
        )
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
