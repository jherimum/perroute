use actix_web::{
    web::{self, Data},
    App, HttpServer, Scope,
};
use anyhow::{Context, Result};
use perroute_backoffice_api::{
    app::AppState,
    routes::{
        channel::{ChannelRouter, CHANNELS_RESOURCE_NAME, CHANNEL_RESOURCE_NAME},
        health::HealthRouter,
        message_type::{
            MessageTypeRouter, MESSAGE_TYPES_RESOURCE_NAME, MESSAGE_TYPE_RESOURCE_NAME,
        },
        route::{RouteRouter, ROUTES_RESOURCE_NAME, ROUTE_RESOURCE_NAME},
        schema::{SchemaRouter, SCHEMAS_RESOURCE_NAME, SCHEMA_RESOURCE_NAME},
        template::{TemplateRouter, TEMPLATES_RESOURCE_NAME, TEMPLATE_RESOURCE_NAME},
    },
};
use perroute_commons::{configuration::settings::Settings, tracing::init_tracing};
use tap::TapFallible;
use tracing_actix_web::TracingLogger;

fn routes(state: AppState) -> Scope {
    let templates = web::scope("/templates")
        .service(
            web::resource("")
                .name(TEMPLATES_RESOURCE_NAME)
                .route(web::get().to(TemplateRouter::query_templates))
                .route(web::post().to(TemplateRouter::create_template)),
        )
        .service(
            web::scope("/{template_id}").service(
                web::resource("")
                    .name(TEMPLATE_RESOURCE_NAME)
                    .route(web::get().to(TemplateRouter::find_template))
                    .route(web::put().to(TemplateRouter::update_template))
                    .route(web::delete().to(TemplateRouter::delete_template)),
            ),
        );

    let schemas = web::scope("/schemas")
        .service(
            web::resource("")
                .name(SCHEMAS_RESOURCE_NAME)
                .route(web::get().to(SchemaRouter::query_schemas))
                .route(web::post().to(SchemaRouter::create_schema)),
        )
        .service(
            web::scope("/{schema_id}").service(
                web::resource("")
                    .name(SCHEMA_RESOURCE_NAME)
                    .route(web::get().to(SchemaRouter::find_schema))
                    .route(web::put().to(SchemaRouter::update_schema))
                    .route(web::delete().to(SchemaRouter::delete_schema)),
            ),
        );

    let message_types = web::scope("/message_types")
        .service(
            web::resource("")
                .name(MESSAGE_TYPES_RESOURCE_NAME)
                .route(web::get().to(MessageTypeRouter::query_message_types))
                .route(web::post().to(MessageTypeRouter::create_message_type)),
        )
        .service(
            web::scope("/{message_type_id}")
                .service(
                    web::resource("")
                        .name(MESSAGE_TYPE_RESOURCE_NAME)
                        .route(web::get().to(MessageTypeRouter::find_message_type))
                        .route(web::put().to(MessageTypeRouter::update_message_type))
                        .route(web::delete().to(MessageTypeRouter::delete_message_type)),
                )
                .service(schemas),
        );

    let routes = web::scope("/routes")
        .service(
            web::resource("")
                .name(ROUTES_RESOURCE_NAME)
                .route(web::get().to(RouteRouter::query_routes))
                .route(web::post().to(RouteRouter::create_route)),
        )
        .service(
            web::scope("/{route_id}").service(
                web::resource("")
                    .name(ROUTE_RESOURCE_NAME)
                    .route(web::get().to(RouteRouter::find_route))
                    .route(web::put().to(RouteRouter::update_route))
                    .route(web::delete().to(RouteRouter::delete_route)),
            ),
        );

    let channels = web::scope("/channels")
        .service(
            web::resource("")
                .name(CHANNELS_RESOURCE_NAME)
                .route(web::get().to(ChannelRouter::query_channels))
                .route(web::post().to(ChannelRouter::create_channel)),
        )
        .service(
            web::scope("/{channel_id}").service(
                web::resource("")
                    .name(CHANNEL_RESOURCE_NAME)
                    .route(web::get().to(ChannelRouter::find_channel))
                    .route(web::put().to(ChannelRouter::update_channel))
                    .route(web::delete().to(ChannelRouter::delete_channel)),
            ),
        );

    web::scope("")
        .service(web::resource("health").route(web::get().to(HealthRouter::health)))
        .service(
            web::scope("/api").service(
                web::scope("/v1")
                    .service(channels)
                    .service(message_types)
                    .service(routes)
                    .service(templates),
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
