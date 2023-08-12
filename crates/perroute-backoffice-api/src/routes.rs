use self::{
    channel::ChannelRouter, health::HealthRouter, message::MessageRouter,
    message_type::MessageTypeRouter, route::RouteRouter, schema::SchemaRouter,
    template::TemplateRouter,
};
use actix_web::{web, Scope};

pub mod channel;
pub mod health;
pub mod message;
pub mod message_type;
pub mod route;
pub mod schema;
pub mod template;

pub fn routes() -> Scope {
    let templates = web::scope("/templates")
        .service(
            web::resource("")
                .name(TemplateRouter::TEMPLATES_RESOURCE_NAME)
                .route(web::get().to(TemplateRouter::query_templates))
                .route(web::post().to(TemplateRouter::create_template)),
        )
        .service(
            web::scope("/{template_id}").service(
                web::resource("")
                    .name(TemplateRouter::TEMPLATE_RESOURCE_NAME)
                    .route(web::get().to(TemplateRouter::find_template))
                    .route(web::put().to(TemplateRouter::update_template))
                    .route(web::delete().to(TemplateRouter::delete_template)),
            ),
        );

    let schemas = web::scope("/schemas")
        .service(
            web::resource("")
                .name(SchemaRouter::SCHEMAS_RESOURCE_NAME)
                .route(web::get().to(SchemaRouter::query_schemas))
                .route(web::post().to(SchemaRouter::create_schema)),
        )
        .service(
            web::scope("/{schema_id}")
                .service(
                    web::resource("")
                        .name(SchemaRouter::SCHEMA_RESOURCE_NAME)
                        .route(web::get().to(SchemaRouter::find_schema))
                        .route(web::put().to(SchemaRouter::update_schema))
                        .route(web::delete().to(SchemaRouter::delete_schema)),
                )
                .service(
                    web::scope("/clone").service(
                        web::resource("")
                            .name(SchemaRouter::SCHEMA_CLONE_RESOURCE_NAME)
                            .route(web::post().to(SchemaRouter::clone)),
                    ),
                ),
        );

    let message_types = web::scope("/message_types")
        .service(
            web::resource("")
                .name(MessageTypeRouter::MESSAGE_TYPES_RESOURCE_NAME)
                .route(web::get().to(MessageTypeRouter::query_message_types))
                .route(web::post().to(MessageTypeRouter::create_message_type)),
        )
        .service(
            web::scope("/{message_type_id}")
                .service(
                    web::resource("")
                        .name(MessageTypeRouter::MESSAGE_TYPE_RESOURCE_NAME)
                        .route(web::get().to(MessageTypeRouter::find_message_type))
                        .route(web::put().to(MessageTypeRouter::update_message_type))
                        .route(web::delete().to(MessageTypeRouter::delete_message_type)),
                )
                .service(schemas),
        );

    let routes = web::scope("/routes")
        .service(
            web::resource("")
                .name(RouteRouter::ROUTES_RESOURCE_NAME)
                .route(web::get().to(RouteRouter::query_routes))
                .route(web::post().to(RouteRouter::create_route)),
        )
        .service(
            web::scope("/{route_id}").service(
                web::resource("")
                    .name(RouteRouter::ROUTE_RESOURCE_NAME)
                    .route(web::get().to(RouteRouter::find_route))
                    .route(web::put().to(RouteRouter::update_route))
                    .route(web::delete().to(RouteRouter::delete_route)),
            ),
        );

    let channels = web::scope("/channels")
        .service(
            web::resource("")
                .name(ChannelRouter::CHANNELS_RESOURCE_NAME)
                .route(web::get().to(ChannelRouter::query_channels))
                .route(web::post().to(ChannelRouter::create_channel)),
        )
        .service(
            web::scope("/{channel_id}").service(
                web::resource("")
                    .name(ChannelRouter::CHANNEL_RESOURCE_NAME)
                    .route(web::get().to(ChannelRouter::find_channel))
                    .route(web::put().to(ChannelRouter::update_channel))
                    .route(web::delete().to(ChannelRouter::delete_channel)),
            ),
        );

    let messages = web::scope("/messages").service(
        web::resource("")
            .name(MessageRouter::MESSAGES_RESOURCE_NAME)
            .route(web::post().to(MessageRouter::create_message)),
    );

    web::scope("")
        .service(
            web::resource(HealthRouter::HEALTH_RESOURCE_NAME)
                .route(web::get().to(HealthRouter::health)),
        )
        .service(
            web::scope("/api").service(
                web::scope("/v1")
                    .service(channels)
                    .service(message_types)
                    .service(messages)
                    .service(templates)
                    .service(routes),
            ),
        )
}
